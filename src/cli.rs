use clap::Parser;

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
}