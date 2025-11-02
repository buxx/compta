use eframe::egui::{self, Frame, RichText, Vec2};
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::{app::scale_buttons, line::Lines};

use super::effect::Effect;

pub fn render(ui: &mut egui::Ui, lines: &mut Lines) -> Vec<Effect> {
    let mut effects = vec![];
    effects.extend(scale_buttons(ui));

    for category in lines.categories() {
        ui.heading(category);

        egui::Grid::new(format!("category_{category}"))
            .striped(true)
            .show(ui, |ui| {
                ui.label("");
                ui.label(RichText::new("Categorie").strong());
                for month in &lines.active_months {
                    ui.label(RichText::new(month.format("%Y-%m").to_string()).strong());
                }
                ui.label(RichText::new("Total").strong());
                ui.label(RichText::new("Moyenne").strong());
                ui.end_row();

                for (category_, sub_category, months, total, average) in
                    lines.sub_categories_total()
                {
                    if category_ == category {
                        if ui.button("👓").clicked() {
                            effects.push(Effect::SelectSubCategory(Some(sub_category.clone())));
                        }
                        ui.label(sub_category);

                        for month_total in months {
                            ui.label(format!("{:>.2}", month_total));
                        }

                        ui.label(format!("{:>.2}", total));
                        ui.label(format!("{:>.2}", average));
                        ui.end_row();
                    }
                }
            });

        ui.separator();
        Frame::default().show(ui, |ui| {
            ui.collapsing(format!("Histogramme {category}"), |ui| {
                ui.set_min_size(Vec2::new(ui.available_width(), 350.0));
                let plot = Plot::new(format!("Historique {category}")).legend(Legend::default());
                let x = plot.show(ui, |plot_ui| {
                    for (category_, sub_category, _, values) in lines.sous_categories_histogram() {
                        if category_ == category {
                            plot_ui.line(Line::new(sub_category, PlotPoints::from(values.clone())));
                        }
                    }
                });
            });
            ui.separator();
        });

        ui.add_space(20.0);
    }

    effects
}
