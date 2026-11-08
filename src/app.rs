use eframe::egui;
use egui::Color32;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::cli::Args;
use crate::ga::{Population, POINT_RADIUS};

const BACKGROUND_COLOR: Color32 = Color32::from_rgb(38, 38, 38);
const POINT_COLOR: Color32 = Color32::from_rgb(140, 200, 255);

pub struct App {
    args: Args,
    population: Population,
    rng: StdRng
}

impl App {
    pub fn new(__cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        let mut rng = StdRng::from_rng(&mut rand::rng());
        let population = Population::random(
            args.population,
            args.width as f32,
            args.height as f32,
            &mut rng,
        );
        Self {
            args, 
            population,
            rng
        }
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
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let painter = ui.painter();

                let sx = rect.width() / self.args.width as f32;
                let sy = rect.height() / self.args.height as f32;

                for ind in &self.population.individuals {
                    let pos = rect.min + egui::vec2(ind.x * sx, ind.y * sy);
                    painter.circle_filled(pos, POINT_RADIUS, POINT_COLOR);
                }
            });
            
    }
}



