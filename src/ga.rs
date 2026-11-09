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

        if ctx.max_dist_sq <= 0.0 {
            return 1.0;
        }

        1.0 - (d_sq / ctx.max_dist_sq).clamp(0.0, 1.0)
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
}

/// Um conjunto de indivíduos que formam uma geração da população.
pub struct Population {
    pub individuals: Vec<Individual>
}

impl Population {
    /// Cria uma nova população com `n` indivíduos distribuídos aleatoriamente.
    pub fn random(n: usize, width: f32, height: f32, rng: &mut impl RngExt) -> Self {
        Self {
            individuals: (0..n)
                .map(|_| Individual::random(width, height, rng))
                .collect()
        }
    }

    /// Evoluimos para a próxima geração
    pub fn evolve_from_best(
        &mut self,
        ctx: &FitnessContext,
        mutation_strength: f32,
        width: f32,
        height: f32,
        rng: &mut impl RngExt
    ) -> GenerationStats {

        let n = self.individuals.len();

        // Inicializamos com o primeiro elemento
        let mut best = self.individuals[0];
        let mut best_fit = best.fitness(ctx);
        
        let mut sum = best_fit;
        let mut sum_sq = best_fit * best_fit;

        for &ind in self.individuals.iter().skip(1) {
            let f = ind.fitness(ctx);

            sum += f;
            sum_sq += f * f;

            if f > best_fit {
                best = ind;
                best_fit = f;
            }
        }

        let mean = sum / n as f32;
        let variance = (sum_sq / n as f32) - (mean * mean);
        let std_dev = variance.max(0.0).sqrt();

        for ind in self.individuals.iter_mut() {
            *ind = best.mutated(
                mutation_strength,
                width,
                height,
                rng
            );
        }

        GenerationStats {
            generation: 0,
            mean_fitness: mean,
            std_dev
        }
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
    pub max_dist_sq: f32
}

impl FitnessContext {
    /// Apenas monta o contexto
    pub fn new(target_x: f32, target_y: f32, max_dist_sq: f32) -> Self {
        Self {
            target_x,
            target_y,
            max_dist_sq,
        }
    }

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


