use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::{app::scale_buttons, line::Lines};

use super::effect::Effect;

pub fn render(ui: &mut egui::Ui, lines: &mut Lines) -> Vec<Effect> {
    let mut effects = vec![];
    effects.extend(scale_buttons(ui));

    egui::Grid::new("categories_totals")
        .striped(true)
        .show(ui, |ui| {
            for (category, months, total, average) in lines.categories_totals() {
                if ui.button("👓").clicked() {
                    effects.push(Effect::SelectCategory(Some(category.clone())));
                }
                ui.label(category);
                ui.label(format!("{:>.2}", total));
                ui.label(format!("{:>.2}", average));
                ui.end_row();
            }
        });

    ui.separator();
    ui.checkbox(
        &mut lines.categories_histogram_display_expenses_only,
        "Dépenses uniquement",
    );

    let plot = Plot::new("Historique").legend(Legend::default());

    let _ = plot.show(ui, |plot_ui| {
        for (category, positive, values) in lines.categories_histogram() {
            if (!lines.categories_histogram_display_expenses_only
                || (lines.categories_histogram_display_expenses_only && !positive))
            {
                plot_ui.line(Line::new(category, PlotPoints::from(values.clone())));
            }
        }
    });

    effects
}
