use chrono::Datelike;
use eframe::egui::{self, RichText};

use crate::{app::scale_buttons, line::Lines};

use super::effect::Effect;

pub fn render<'a>(
    ui: &mut egui::Ui,
    lines: &mut Lines,
    selected_category: &'a Option<String>,
    selected_sub_category: &'a Option<String>,
    filter_text: &'a str,
) -> Vec<Effect> {
    let mut effects = vec![];
    effects.extend(scale_buttons(ui));

    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_label("CategoryFilter")
            .selected_text(selected_category.clone().unwrap_or_default())
            .show_ui(ui, |ui| {
                let mut selected_category_ = selected_category.clone();
                ui.selectable_value(&mut selected_category_, None, "".to_string());
                for category in lines.categories() {
                    ui.selectable_value(&mut selected_category_, Some(category.clone()), category);
                }
                if &selected_category_ != selected_category {
                    effects.push(Effect::SelectCategory(selected_category_.clone()))
                }
            });

        ui.separator();

        egui::ComboBox::from_label("SubCategoryFilter")
            .selected_text(selected_sub_category.clone().unwrap_or_default())
            .show_ui(ui, |ui| {
                let mut sub_selected_category_ = selected_sub_category.clone();
                ui.selectable_value(&mut sub_selected_category_, None, "".to_string());
                for (_, sub_category) in lines.sub_categories() {
                    ui.selectable_value(
                        &mut sub_selected_category_,
                        Some(sub_category.clone()),
                        sub_category,
                    );
                }
                if &sub_selected_category_ != selected_sub_category {
                    effects.push(Effect::SelectSubCategory(sub_selected_category_.clone()))
                }
            });

        ui.separator();

        let mut filter_text_ = filter_text.to_string();
        if ui
            .add(egui::TextEdit::singleline(&mut filter_text_))
            .changed()
        {
            effects.push(Effect::SetFilterText(filter_text_.to_string()));
        }

        ui.separator();
    });

    ui.separator();
    ui.add_space(20.0);

    let mut current_date: Option<chrono::NaiveDate> = None;
    let mut current_debit = 0.0;
    let mut current_credit = 0.0;

    egui::Grid::new("lines").striped(true).show(ui, |ui| {
        for line in lines.lines() {
            if let Some(selected_category) = selected_category {
                if line.categorie() != selected_category {
                    continue;
                }
            }

            if let Some(selected_sub_category) = selected_sub_category {
                if line.sous_categorie() != selected_sub_category {
                    continue;
                }
            }

            if !filter_text.is_empty()
                && !line
                    .libelle_simplifie()
                    .to_lowercase()
                    .contains(&filter_text.to_lowercase())
                && !line
                    .libelle_operation()
                    .to_lowercase()
                    .contains(&filter_text.to_lowercase())
                && !line
                    .categorie()
                    .to_lowercase()
                    .contains(&filter_text.to_lowercase())
                && !line
                    .sous_categorie()
                    .to_lowercase()
                    .contains(&filter_text.to_lowercase())
                && !line
                    .debit()
                    .map(|v| format!("{:>.2}", v))
                    .unwrap_or("".to_string())
                    .contains(&filter_text.to_lowercase())
                && !line
                    .credit()
                    .map(|v| format!("{:>.2}", v))
                    .unwrap_or("".to_string())
                    .contains(&filter_text.to_lowercase())
            {
                continue;
            }

            let date = {
                let mut splitted = line.date_raw().split('/');
                let _ = splitted.next().unwrap().parse::<u32>().unwrap();
                let month = splitted.next().unwrap().parse::<u32>().unwrap();
                let year = splitted.next().unwrap().parse::<i32>().unwrap();
                chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap()
            };

            if let Some(current_date) = current_date {
                if date.year() != current_date.year() || date.month() != current_date.month() {
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    ui.label(RichText::new(format!("{:>.2}", current_debit)).strong());
                    ui.label(RichText::new(format!("{:>.2}", current_credit)).strong());
                    ui.end_row();
                    current_debit = 0.0;
                    current_credit = 0.0;
                }
            }

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

            current_date = Some(date);
            current_debit += line.debit().unwrap_or(0.0);
            current_credit += line.credit().unwrap_or(0.0);
        }

        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label("");
        ui.label(RichText::new(format!("{:>.2}", current_debit)).strong());
        ui.label(RichText::new(format!("{:>.2}", current_credit)).strong());
        ui.end_row();
    });

    effects
}
