use eframe::egui;
use egui::Color32;

use crate::cli::Args;


const BACKGROUND_COLOR: Color32 = Color32::from_rgb(38, 38, 38);

pub struct App {
    args: Args,
}

impl App {
    pub fn new(__cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        Self {args}
    }
}

impl eframe::App for App {
    /// Atualizará a janela continuamente
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Criamos um painel
        egui::CentralPanel::default()
            // Fazemos especificações nele
            .frame(
                egui::Frame::default()
                    .fill(BACKGROUND_COLOR)
                    .inner_margin(egui::Margin::same(8)),
            )
            // Desenhamos ele na janela
            .show(ctx, |_ui| {
                
            });
            
    }
}



