use eframe::egui;

use crate::line::Lines;

use super::effect::Effect;

pub fn render<'a>(ui: &mut egui::Ui, lines: &mut Lines) -> Vec<Effect> {
    egui::Grid::new("lines").striped(true).show(ui, |ui| {
        for line in lines.recurring() {
            ui.label(line.date_raw());
            ui.label(line.libelle_simplifie());
            ui.label(line.libelle_operation());
            ui.label(line.categorie());
            ui.label(line.sous_categorie());
            ui.label(
                line.debit()
                    .map(|v| format!("{:>.2}", v))
                    .unwrap_or("".to_string()),
            );
            ui.label(
                line.credit()
                    .map(|v| format!("{:>.2}", v))
                    .unwrap_or("".to_string()),
            );
            ui.end_row();
        }
    });

    vec![]
}
