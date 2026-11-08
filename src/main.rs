mod app;
mod cli;
mod ga;

use clap::Parser;

fn main() -> eframe::Result<()> {
    let args = cli::Args::parse();

    let viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([args.width as f32, args.height as f32])
        .with_title("Algoritmos Genéticos");

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        // Não é o título da janela nem do binário, trata-se de um app_id
        "ga-viz",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc, args)))),
    )
}