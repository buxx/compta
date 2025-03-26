use std::{collections::HashSet, fs};

use eframe::egui;
use egui_commonmark::*;
use egui_file_dialog::FileDialog;

const SIGNIFICANT: f32 = 175.0;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MaCompta🦀",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    file_dialog: FileDialog,
    cache: CommonMarkCache,
    result: Option<String>,
    scale_factor: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            cache: Default::default(),
            result: Default::default(),
            scale_factor: 1.5,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.scale_factor);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("🔹").clicked() {
                    self.scale_factor /= 1.1;
                }
                if ui.button("🔷").clicked() {
                    self.scale_factor *= 1.1;
                }

                if ui.button("Sélectionner un fichier").clicked() {
                    self.file_dialog.pick_file();
                }
            });
            self.file_dialog.update(ctx);

            if let Some(path) = self.file_dialog.take_picked() {
                let raw = fs::read_to_string(path).expect("Failed to read file");
                self.result = Some(render(&raw));
            }

            if let Some(result) = &self.result {
                ui.separator();
                egui::ScrollArea::both().show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut self.cache, result);
                });
            }
        });
    }
}

fn render(raw: &str) -> String {
    let mut result = String::new();
    let headers: Vec<&str> = raw.lines().next().unwrap().split(";").collect();

    let debit_index = find_index(&headers, "Debit");
    let category_index = find_index(&headers, "Categorie");
    let sub_category_index = find_index(&headers, "Sous categorie");

    let categories = extract_unique_values(raw, category_index);

    result.push_str("# ➡ Par catégories\n");
    result.push('\n');
    result.push_str(&print_totals(raw, debit_index, category_index, &categories));

    result.push('\n');
    result.push_str("# ➡ Par sous catégories\n");
    result.push('\n');
    result.push_str(&print_totals2(
        raw,
        debit_index,
        category_index,
        categories,
        sub_category_index,
    ));

    result.push('\n');
    result.push_str(&format!("# 🔥 Dépenses >= {:>.2}\n", SIGNIFICANT));
    result.push('\n');

    let mut significatives = raw
        .lines()
        .skip(1)
        .filter_map(|line| {
            let debit_str = line.split(";").nth(debit_index).unwrap();
            if !debit_str.trim().is_empty() {
                Some((line, debit_str.replace(",", ".").parse::<f32>().unwrap()))
            } else {
                None
            }
        })
        .filter(|(_, v)| (v.abs() * 100.) as i32 >= (SIGNIFICANT * 100.) as i32)
        .collect::<Vec<(&str, f32)>>();
    significatives.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));

    result.push_str(&format!(
        "|{}|\n",
        raw.lines().next().unwrap().replace(";", "|")
    ));
    result.push_str(&format!(
        "|{}|\n",
        raw.lines()
            .next()
            .unwrap()
            .split(";")
            .map(|_| "-")
            .collect::<Vec<&str>>()
            .join("|")
    ));
    for (line, _v) in significatives {
        result.push_str(&format!("|{}|\n", line.replace(";", "|")))
    }

    result
}

fn find_index(headers: &[&str], target: &str) -> usize {
    headers
        .iter()
        .position(|&h| h == target)
        .expect("Header not found")
}

fn extract_unique_values(raw: &str, index: usize) -> Vec<String> {
    raw.lines()
        .skip(1)
        .filter_map(|line| line.split(";").nth(index).map(String::from))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn extract_unique_values2(
    raw: &str,
    index1: usize,
    index1_value: &str,
    index2: usize,
) -> Vec<String> {
    raw.lines()
        .skip(1)
        .filter(|line| line.split(";").nth(index1).map(String::from).unwrap() == index1_value)
        .filter_map(|line| line.split(";").nth(index2).map(String::from))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn print_totals(raw: &str, debit_index: usize, index: usize, values: &[String]) -> String {
    let mut result = String::new();
    let mut totals = vec![];

    for value in values {
        let total: f32 = raw
            .lines()
            .skip(1)
            .filter(|line| line.split(";").nth(index).unwrap() == value)
            .filter_map(|line| {
                let debit_str = line.split(";").nth(debit_index).unwrap();
                if !debit_str.trim().is_empty() {
                    debit_str.replace(",", ".").parse::<f32>().ok()
                } else {
                    None
                }
            })
            .sum();

        totals.push((value, total));
        totals.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
    }

    result.push_str("||||\n");
    result.push_str("|-|-|-|\n");
    for (name, total) in totals {
        result.push_str(&format!("|{}|{:>.2}|\n", name, total.abs()));
    }

    result
}

fn print_totals2(
    raw: &str,
    debit_index: usize,
    category_index: usize,
    categories: Vec<String>,
    sub_category_index: usize,
) -> String {
    let mut result = String::new();

    for category in categories {
        result.push_str(&format!("## {}\n", category));
        result.push_str("||||\n");
        result.push_str("|-|-|-|\n");

        let sub_categories =
        extract_unique_values2(raw, category_index, &category, sub_category_index);
        
        let mut totals = vec![];
        for sub_category in sub_categories {
            let total: f32 = raw
                .lines()
                .skip(1)
                .filter(|line| line.split(";").nth(sub_category_index).unwrap() == sub_category)
                .filter_map(|line| {
                    let debit_str = line.split(";").nth(debit_index).unwrap();
                    if !debit_str.trim().is_empty() {
                        debit_str.replace(",", ".").parse::<f32>().ok()
                    } else {
                        None
                    }
                })
                .sum();

            totals.push((sub_category, total));
            totals.sort_by(|a, b| ((a.1 * 1000.) as i32).cmp(&((b.1 * 1000.) as i32)));
        }

        for (name, total) in &totals {
            result.push_str(&format!("|{}|{:>.2}|\n", name, total.abs()));
        }
    }

    result
}
