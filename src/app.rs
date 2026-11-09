use std::path::PathBuf;

use eframe::egui;
use egui::Color32;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::cli::{Args, EvolutionMode};
use crate::ga::{FitnessContext, POINT_RADIUS, Population};
use crate::stats::append_to_csv;

const BACKGROUND_COLOR: Color32 = Color32::from_rgb(38, 38, 38);

/// Mapeamos o fitness para um gradiente de cores
pub fn heat_color(t: f32) -> Color32 {
    const STOPS: &[(f32, Color32)] = &[
        (0.00, Color32::from_rgb( 48,  18,  59)),
        (0.20, Color32::from_rgb( 70, 107, 227)),
        (0.40, Color32::from_rgb( 40, 187, 220)),
        (0.60, Color32::from_rgb( 84, 216,  90)),
        (0.80, Color32::from_rgb(240, 190,  40)),
        (1.00, Color32::from_rgb(224,  53,  35)),
    ];

    let t = t.clamp(0.0, 1.0);
    for w in STOPS.windows(2) {
        let (t1, c1) = w[1];
        if t <= t1 {
            let (t0, c0) = w[0];
            return c0.lerp_to_gamma(c1, (t - t0) / (t1 - t0));
        }
    }
    STOPS[STOPS.len() - 1].1
}

pub struct App {
    args: Args,
    population: Population,
    rng: StdRng,
    target_x: f32,
    target_y: f32,
    max_dist_sq: f32,
    inv_max_dist_sq: f32,
    generation: usize,
    csv_path: PathBuf,
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

        let w = args.width as f32;
        let h = args.height as f32;
        let target_x = w * 0.5;
        let target_y = h * 0.5;

        let max_dist_sq = FitnessContext::calc_max_dist_sq(target_x, target_y, w, h);
        let inv_max_dist_sq = if max_dist_sq > 0.0 { 1.0 / max_dist_sq } else { 1.0 };

        let csv_path = PathBuf::from("evolution.csv");
        if let Err(e) = std::fs::File::create(&csv_path) {
            eprintln!("Aviso: não foi possível limpar {:?}: {}", csv_path, e);
        }

        Self {
            args,
            population,
            rng,
            target_x,
            target_y,
            max_dist_sq,
            inv_max_dist_sq,
            generation: 0,
            csv_path,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(BACKGROUND_COLOR)
                    .inner_margin(egui::Margin::same(8)),
            )
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let painter = ui.painter();
                let w = self.args.width as f32;
                let h = self.args.height as f32;
                let sx = rect.width() / w;
                let sy = rect.height() / h;

                if let Some(p) = ctx.input(|i| i.pointer.hover_pos()) {
                    if rect.contains(p) {
                        let nx = ((p.x - rect.min.x) / sx).clamp(0.0, w);
                        let ny = ((p.y - rect.min.y) / sy).clamp(0.0, h);

                        if nx != self.target_x || ny != self.target_y {
                            self.target_x = nx;
                            self.target_y = ny;
                            self.max_dist_sq =
                                FitnessContext::calc_max_dist_sq(nx, ny, w, h);
                            self.inv_max_dist_sq = if self.max_dist_sq > 0.0 {
                                1.0 / self.max_dist_sq
                            } else {
                                1.0
                            };
                        }
                    }
                }

                let fit_ctx = FitnessContext {
                    target_x: self.target_x,
                    target_y: self.target_y,
                    max_dist_sq: self.max_dist_sq,
                    inv_max_dist_sq: self.inv_max_dist_sq,
                };

                if ctx.input(|i| i.key_pressed(egui::Key::A)) {
                    let mut stats = match self.args.evolution {
                        EvolutionMode::Asexual => self.population.evolve_asexual(
                            &fit_ctx,
                            self.args.mutation,
                            w,
                            h,
                            &mut self.rng,
                        ),
                        EvolutionMode::Sexual => self.population.evolve_sexual(
                            &fit_ctx,
                            self.args.parent_pool,
                            self.args.mutation,
                            w,
                            h,
                            &mut self.rng,
                        ),
                    };

                    stats.generation = self.generation;
                    self.generation += 1;

                    if let Err(e) = append_to_csv(&self.csv_path, &stats) {
                        eprintln!("Erro ao salvar em csv: {}", e);
                    }
                }

                for ind in self.population.iter() {
                    let pos = rect.min + egui::vec2(ind.x * sx, ind.y * sy);
                    let t = ind.fitness(&fit_ctx);
                    painter.circle_filled(pos, POINT_RADIUS, heat_color(t));
                }
            });
    }
}