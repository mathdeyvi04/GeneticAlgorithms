use rand::RngExt;

use crate::stats::GenerationStats;

/// Raio visual (em pixels) usado para desenhar cada indivíduo na tela.
pub const POINT_RADIUS: f32 = 3.0;

#[derive(Clone, Copy, Debug)]
pub struct Individual {
    pub x: f32,
    pub y: f32
}

impl Individual {
    /// Gera um indivíduo com coordenadas aleatórias dentro dos limites fornecidos.
    pub fn random(width: f32, height: f32, rng: &mut impl RngExt) -> Self {
        // Usamos impl Rng para que seja aceito qualquer tipo que implemente o trait Rng
        Self {
            x: rng.random_range(0.0..width),
            y: rng.random_range(0.0..height)
        }
    }

    /// Distância Quadrada do Ponto Individual a um ponto específico
    pub fn dist_sq_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        dx * dx + dy * dy
    }

    /// Fitness que consideramos para esse projeto
    pub fn fitness(&self, ctx: &FitnessContext) -> f32 {
        let d_sq = self.dist_sq_to(ctx.target_x, ctx.target_y);
        (1.0 - d_sq * ctx.inv_max_dist_sq).clamp(0.0, 1.0)
    }

    /// Realizamos a reprodução assexual da população
    pub fn mutated(
        &self,
        strength: f32,
        width: f32,
        height: f32,
        rng: &mut impl RngExt
    ) -> Self {
        let s = strength.abs();
        Self {
            x: (self.x + rng.random_range(-s..s)).clamp(0.0, width),
            y: (self.y + rng.random_range(-s..s)).clamp(0.0, height)
        }
    }

    /// Realiza o cruzamento entre dois indivíduos
    pub fn crossover(&self, other: &Individual, rng: &mut impl RngExt) -> Self {
        let ax = rng.random_range(0.0..1.0);
        let ay = rng.random_range(0.0..1.0);
        Self {
            x: ax * self.x + (1.0 - ax) * other.x,
            y: ay * self.y + (1.0 - ay) * other.y,
        }
    }
}

/// Um conjunto de indivíduos que formam uma geração da população.
pub struct Population {
    pub individuals: Vec<Individual>,
    /// Scratch reutilizado por `evolve_sexual` para o ranking (N × 12 bytes).
    scratch_ranked: Vec<(Individual, f32)>,
    /// Scratch reutilizado por `evolve_sexual` para a próxima geração.
    scratch_next: Vec<Individual>,
}

impl Population {
    /// Cria uma nova população com `n` indivíduos distribuídos aleatoriamente.
    pub fn random(n: usize, width: f32, height: f32, rng: &mut impl RngExt) -> Self {
        Self {
            individuals: (0..n)
                .map(|_| Individual::random(width, height, rng))
                .collect(),
            scratch_ranked: Vec::with_capacity(n),
            scratch_next: Vec::with_capacity(n),
        }
    }

    /// Registramos informações da geração assexuada e avançamos a evolução
    #[inline]
    fn snapshot_stats(&self, ctx: &FitnessContext) -> (Individual, GenerationStats) {
        let n = self.individuals.len();
        let mut best = self.individuals[0];
        let mut best_fit = best.fitness(ctx);
        let mut sum = best_fit;
        let mut sum_sq = best_fit * best_fit;
    
        for &ind in self.individuals.iter().skip(1) {
            let f = ind.fitness(ctx);
            sum += f;
            sum_sq += f * f;
            if f > best_fit { best = ind; best_fit = f; }
        }
    
        let mean = sum / n as f32;
        let variance = (sum_sq / n as f32) - mean * mean;
        let std_dev = variance.max(0.0).sqrt();
    
        (best, GenerationStats { generation: 0, mean_fitness: mean, std_dev })
    }

    /// Evoluimos para a próxima geração
    pub fn evolve_asexual(
        &mut self,
        ctx: &FitnessContext,
        mutation_strength: f32,
        width: f32,
        height: f32,
        rng: &mut impl RngExt,
    ) -> GenerationStats {
        let (best, stats) = self.snapshot_stats(ctx);
        for ind in self.individuals.iter_mut() {
            *ind = best.mutated(mutation_strength, width, height, rng);
        }
        stats
    }

    pub fn evolve_sexual(
            &mut self,
            ctx: &FitnessContext,
            parent_pool: usize,
            mutation_strength: f32,
            width: f32,
            height: f32,
            rng: &mut impl RngExt,
        ) -> GenerationStats {
            let n = self.individuals.len();
            if n < 2 {
                return self.evolve_asexual(ctx, mutation_strength, width, height, rng);
            }
    
            // Destructuring: dá &mut disjunto a cada campo. Sem isso, o borrow
            // checker não permite mutar `scratch_ranked` enquanto `individuals`
            // está sendo lido — mesmo sendo campos distintos.
            let Self { individuals, scratch_ranked, scratch_next } = self;
    
            // ── Passada única: fitness + ranking, reusando o buffer ──
            scratch_ranked.clear();
            let mut sum = 0.0f32;
            let mut sum_sq = 0.0f32;
            for &ind in individuals.iter() {
                let f = ind.fitness(ctx);
                sum += f;
                sum_sq += f * f;
                scratch_ranked.push((ind, f));
            }
            let mean = sum / n as f32;
            let variance = (sum_sq / n as f32) - mean * mean;
            let std_dev = variance.max(0.0).sqrt();
    
            // ── Top-K em O(n), sem ordenar o vetor inteiro ──
            let pool_size = parent_pool.clamp(2, n);
            if pool_size < n {
                scratch_ranked.select_nth_unstable_by(pool_size - 1, |a, b| b.1.total_cmp(&a.1));
            }
            let parents = &scratch_ranked[..pool_size];
    
            // ── Nova geração, reusando scratch_next (capacidade já é >= n) ──
            scratch_next.clear();
            while scratch_next.len() < n {
                let i1 = rng.random_range(0..pool_size);
                let i2 = loop {
                    let j = rng.random_range(0..pool_size);
                    if j != i1 { break j; }
                };
                let child = parents[i1]
                    .0
                    .crossover(&parents[i2].0, rng)
                    .mutated(mutation_strength, width, height, rng);
                scratch_next.push(child);
            }
    
            // Troca de buffers: `individuals` recebe a nova geração; `scratch_next`
            // guarda a geração anterior, pronta para ser limpa e reutilizada.
            std::mem::swap(individuals, scratch_next);
    
            GenerationStats { generation: 0, mean_fitness: mean, std_dev }
        }

    /// Itera sobre os indivíduos sem expor o `Vec` interno.
    pub fn iter(&self) -> impl Iterator<Item = &Individual> + '_ {
        // `impl Iterator<...>`: Retorna um tipo anônimo que implementa a trait Iterator.
        // `Item = &Individual`: Garante que cada item gerado seja uma referência (evita clonar).
        // `+ '_`: Amarra o tempo de vida do iterador ao tempo de vida de `&self`.
        //         Ou seja, o iterador não pode viver mais do que a própria Population.
        self.individuals.iter()
    }
}

/// Contexto do Ambiente a partir do qual definiremos o quão bom é um indivíduo
pub struct FitnessContext {
    pub target_x: f32,
    pub target_y:f32,
    pub max_dist_sq: f32,
    pub inv_max_dist_sq: f32
}

impl FitnessContext {

    /// Calcula a maior distância quadrada possível para o tamanho de tela dado.
    pub fn calc_max_dist_sq(target_x: f32, target_y: f32, width: f32, height: f32) -> f32 {
        let dist_sq = |x: f32, y: f32| {
            let dx = x - target_x;
            let dy = y - target_y;
            dx * dx + dy * dy
        };

        // O pior caso de distância será sempre em um dos 4 cantos da tela
        dist_sq(0.0, 0.0)
            .max(dist_sq(width, 0.0))
            .max(dist_sq(0.0, height))
            .max(dist_sq(width, height))
    }
}


