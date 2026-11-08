use rand::RngExt;

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

    /// Retorna o tamanho atual da população.
    pub fn len(&self) -> usize {
        self.individuals.len()
    }
}


