use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
pub enum EvolutionMode {
    /// Evolução Assexuada
    Asexual,
    /// Evolução Sexual
    Sexual
}

/// Controlador de Argumentos 
#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about)]
pub struct Args {
    /// Largura janela 
    #[arg(long, default_value_t = 800)]
    pub width: u32,

    /// Altura janela 
    #[arg(long, default_value_t = 600)]
    pub height: u32,

    /// Número de Indivíduos em cada População 
    #[arg(long, default_value_t = 100)]
    pub population: usize,

    /// Intensidade de mutação, em unidades do espaço de busca (por gene).
    #[arg(long, default_value_t = 20.0)]
    pub mutation: f32,

    /// Modo de evolução entre gerações.
    #[arg(long, value_enum, default_value = "asexual")]
    pub evolution: EvolutionMode,

    /// Tamanho do pool de pais (só usado no modo `sexual`).
    #[arg(long, default_value_t = 3)]
    pub parent_pool: usize,
}