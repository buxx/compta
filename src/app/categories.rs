use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::{app::scale_buttons, line::Lines};

use super::effect::Effect;

pub fn render(ui: &mut egui::Ui, lines: &Lines) -> Vec<Effect> {
    let mut effects = vec![];
    effects.extend(scale_buttons(ui));

    egui::Grid::new("categories_totals")
        .striped(true)
        .show(ui, |ui| {
            for (category, total) in lines.categories_totals() {
                if ui.button("👓").clicked() {
                    effects.push(Effect::SelectCategory(Some(category.clone())));
                }
                ui.label(category);
                ui.label(format!("{:>.2}", total));
                ui.end_row();
            }
        });

    ui.separator();

    let plot = Plot::new("Historique").legend(Legend::default());

    let _ = plot.show(ui, |plot_ui| {
        for (category, values) in lines.categories_histogram() {
            plot_ui.line(Line::new(category, PlotPoints::from(values.clone())));
        }
    });

    effects
}
