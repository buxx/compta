use std::{fs, path::PathBuf};

use derive_more::{Constructor, Display};
use effect::Effect;
use eframe::egui::{self, Align, Layout};
use egui_dock::{DockArea, DockState, Style};
use egui_file_dialog::FileDialog;

pub mod categories;
pub mod effect;
pub mod home;
pub mod lines;
pub mod recurring;
pub mod sub_categories;

use crate::{
    extract::{TryIntoLines, extract_recuring},
    line::Lines,
};

pub struct MyApp {
    start_from: Option<PathBuf>,
    file_dialog: FileDialog,
    lines: Option<Lines>,
    scale_factor: f32,
    tree: DockState<Tab>,
    selected_category: Option<String>,
    selected_sub_category: Option<String>,
    filter_text: String,
}

fn dock() -> DockState<Tab> {
    DockState::new(vec![
        Tab::Home,
        Tab::Categories,
        Tab::SubCategories,
        Tab::Lines,
        Tab::Recurring,
    ])
}

impl MyApp {
    pub fn new(start_from: Option<PathBuf>) -> Self {
        Self {
            start_from,
            file_dialog: Default::default(),
            lines: Default::default(),
            scale_factor: 1.5,
            tree: dock(),
            selected_category: Default::default(),
            selected_sub_category: Default::default(),
            filter_text: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.scale_factor);

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.file_dialog.update(ctx);

            if let Some(path) = self.file_dialog.take_picked() {
                let raw = fs::read(&path).expect("Failed to read file");
                let raw = String::from_utf8_lossy(&raw).to_string();
                let name = path
                    .file_stem()
                    .map(|n| n.display().to_string())
                    .unwrap_or("data.csv".to_string());
                self.lines = Some(raw.into_lines(name).unwrap());
                self.tree = dock();
            }

            if let Some(path) = &self.start_from {
                let raw = fs::read(path).expect("Failed to read file");
                let raw = String::from_utf8_lossy(&raw).to_string();
                let name = path
                    .file_stem()
                    .map(|n| n.display().to_string())
                    .unwrap_or("data.csv".to_string());
                self.lines = Some(raw.into_lines(name).unwrap());
                self.tree = dock();
                self.start_from = None;
            }

            let mut effects: Vec<Effect> = vec![];

            DockArea::new(&mut self.tree)
                .show_close_buttons(false)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show(
                    ctx,
                    &mut TabViewer::new(
                        &mut self.lines,
                        &mut self.file_dialog,
                        &mut effects,
                        &self.selected_category,
                        &self.selected_sub_category,
                        &self.filter_text,
                    ),
                );

            while let Some(effect) = effects.pop() {
                match effect {
                    Effect::IncreaseScale => {
                        self.scale_factor *= 1.1;
                    }
                    Effect::DecreaseScale => {
                        self.scale_factor /= 1.1;
                    }
                    Effect::SelectCategory(category) => {
                        self.selected_category = category;
                        self.selected_sub_category = None;
                    }
                    Effect::SelectSubCategory(sub_category) => {
                        self.selected_category = None;
                        self.selected_sub_category = sub_category;
                    }
                    Effect::SetFilterText(value) => {
                        self.filter_text = value;
                    }
                    Effect::ClearLines => {
                        self.lines = None;
                    }
                    Effect::RecomputeRecurring => {
                        if let Some(lines) = &mut self.lines {
                            lines.recurring = extract_recuring(lines);
                        }
                    }
                }
            }
        });
    }
}

#[derive(Debug, Display)]
enum Tab {
    Home,
    Categories,
    SubCategories,
    Lines,
    Recurring,
}

#[derive(Constructor)]
struct TabViewer<'a> {
    lines: &'a mut Option<Lines>,
    file_dialog: &'a mut FileDialog,
    messages: &'a mut Vec<Effect>,
    selected_category: &'a Option<String>,
    selected_sub_category: &'a Option<String>,
    filter_text: &'a String,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (*tab).to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.messages.extend(match tab {
            Tab::Home => home::render(ui, self.file_dialog, self.lines),
            Tab::Categories => {
                if let Some(lines) = self.lines {
                    categories::render(ui, lines)
                } else {
                    vec![]
                }
            }
            Tab::SubCategories => {
                if let Some(lines) = self.lines {
                    sub_categories::render(ui, lines)
                } else {
                    vec![]
                }
            }
            Tab::Lines => {
                if let Some(lines) = self.lines {
                    lines::render(
                        ui,
                        lines,
                        self.selected_category,
                        self.selected_sub_category,
                        self.filter_text,
                    )
                } else {
                    vec![]
                }
            }
            Tab::Recurring => {
                if let Some(lines) = self.lines {
                    recurring::render(ui, lines)
                } else {
                    vec![]
                }
            }
        });
    }
}

pub fn scale_buttons(ui: &mut egui::Ui) -> Vec<Effect> {
    let mut effects = vec![];

    ui.horizontal_wrapped(|ui| {
        if ui.button("🔹").clicked() {
            effects.push(Effect::DecreaseScale);
        }
        if ui.button("🔷").clicked() {
            effects.push(Effect::IncreaseScale);
        }
    });

    effects
}
