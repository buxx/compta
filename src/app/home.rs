use eframe::egui::{self, Align, Layout};
use egui_file_dialog::FileDialog;
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::line::Lines;

use super::effect::Effect;

pub fn render(
    ui: &mut egui::Ui,
    file_dialog: &mut FileDialog,
    lines: &mut Option<Lines>,
) -> Vec<Effect> {
    let mut effects = vec![];

    ui.with_layout(
        Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            match lines {
                Some(lines) => {
                    //
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        ui.label(format!("Data loaded : {}", lines.name()));
                        if ui.button("Clear").clicked() {
                            effects.push(Effect::ClearLines);
                        };

                        ui.separator();

                        egui::Grid::new("lines").striped(true).show(ui, |ui| {
                            for (date, sum) in lines.months_sums() {
                                ui.label(date.format("%Y-%m").to_string());
                                ui.label(format!("{sum}"));
                                ui.end_row();
                            }
                        });

                        ui.separator();

                        ui.collapsing("Histogramme", |ui| {
                            ui.checkbox(
                                &mut lines.categories_histogram_display_expenses_only,
                                "Dépenses uniquement",
                            );

                            let plot = Plot::new("Historique").legend(Legend::default());

                            let _ = plot.show(ui, |plot_ui| {
                                plot_ui.line(Line::new(
                                    "Histogramme",
                                    PlotPoints::from(
                                        lines
                                            .months_sums()
                                            .iter()
                                            .enumerate()
                                            .map(|(i, (_, v))| [i as f64, *v as f64])
                                            .collect::<Vec<[f64; 2]>>(),
                                    ),
                                ));
                            });
                        });
                    });
                }
                None => {
                    //
                    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                        if ui.button("Sélectionner un fichier").clicked() {
                            file_dialog.pick_file();
                        };
                    });
                }
            };
        },
    );

    effects
}
