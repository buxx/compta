use eframe::egui::{self, Frame, Vec2};
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
                for (category_, sub_category, total) in lines.sub_categories_total() {
                    if category_ == category {
                        if ui.button("👓").clicked() {
                            effects.push(Effect::SelectSubCategory(Some(sub_category.clone())));
                        }
                        ui.label(sub_category);
                        ui.label(format!("{:>.2}", total));
                        ui.label(format!("{:>.2}", total / lines.months() as f32));
                        ui.end_row();
                    }
                }
            });

        ui.separator();

        Frame::default().show(ui, |ui| {
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
        ui.add_space(20.0);
    }

    effects
}
