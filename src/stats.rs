use std::fs::{OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Estatísticas de uma única geração.
#[derive(Debug, Clone, Copy)]
pub struct GenerationStats {
    pub generation: usize,
    pub mean_fitness: f32,
    pub std_dev: f32,
}

/// Salvamos informações de cada geração
pub fn append_to_csv(
    path: impl AsRef<Path>,
    stats: &GenerationStats
) -> std::io::Result<()> {
    let path = path.as_ref();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let mut writer = BufWriter::new(file);

    if !path.exists() || std::fs::metadata(path)?.len() == 0 {
        writeln!(writer, "geracao,media_fitness,desvio_padrao_fitness")?;
    }

    writeln!(
        writer,
        "{},{:.4},{:.4}",
        stats.generation, stats.mean_fitness, stats.std_dev
    )?;

    Ok(())
}