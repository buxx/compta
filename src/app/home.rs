use eframe::egui::{self, Align, Layout};
use egui_file_dialog::FileDialog;

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
