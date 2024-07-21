#![allow(dead_code)]
mod board;
mod bot;
mod gui;
mod helpers;
mod moves;
mod pieces;
mod utils;

use crate::gui::ChessApp;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let window_size = (600., 600.);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(window_size),
        // .with_resizable(false),
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
