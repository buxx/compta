use std::path::PathBuf;

use app::MyApp;
use clap::Parser;
use eframe::egui;

mod app;
mod extract;
mod line;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    lines_path: Option<PathBuf>,
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MaCompta🦀",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(args.lines_path.clone())))
        }),
    )
}
