#![allow(dead_code)]
mod board;
mod gui;
mod helpers;
mod moves;
mod pieces;
mod utils;

use crate::gui::ChessApp;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let window_size = (400., 400.);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_min_inner_size(window_size),
        ..Default::default()
    };

    eframe::run_native(
        "Chess",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<ChessApp>::default()
        }),
    )
}
