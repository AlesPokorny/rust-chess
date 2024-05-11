use eframe::egui::{Color32, Image, Pos2, Rect, Rounding, Shape, Vec2};
use crate::pieces::{PieceKind, Color};

use std::collections::HashMap;

pub fn make_square(rect: Rect, color: Color32) -> Shape {
    Shape::rect_filled(rect, Rounding::ZERO, color)
}

pub fn init_assets<'a>(size: f32) -> HashMap<(PieceKind, Color), Image<'a>> {
    let mut assets: HashMap<(PieceKind, Color), Image> = HashMap::new();

    assets.insert((PieceKind::K, Color::White), Image::from_uri("file://resources/kw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::R, Color::White), Image::from_uri("file://resources/rw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::N, Color::White), Image::from_uri("file://resources/nw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::B, Color::White), Image::from_uri("file://resources/bw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::Q, Color::White), Image::from_uri("file://resources/qw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::P, Color::White), Image::from_uri("file://resources/pw.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::K, Color::Black), Image::from_uri("file://resources/kb.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::R, Color::Black), Image::from_uri("file://resources/rb.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::N, Color::Black), Image::from_uri("file://resources/nb.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::B, Color::Black), Image::from_uri("file://resources/bb.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::Q, Color::Black), Image::from_uri("file://resources/qb.png").fit_to_exact_size(Vec2::new(size, size)));
    assets.insert((PieceKind::P, Color::Black), Image::from_uri("file://resources/pb.png").fit_to_exact_size(Vec2::new(size, size)));

    assets

}