use crate::helpers::Position;
use crate::pieces::{Color, PieceKind};
use eframe::egui::{Color32, Image, Pos2, Rect, Rounding, Shape, Stroke, Vec2};

use std::collections::HashMap;

pub fn make_square(rect: Rect, color: Color32, fill: bool) -> Shape {
    if fill {
        return Shape::rect_filled(rect, Rounding::ZERO, color);
    }

    Shape::rect_stroke(rect, Rounding::ZERO, Stroke::new(3.0, color))
}

pub fn init_assets<'a>(size: f32) -> HashMap<(PieceKind, Color), Image<'a>> {
    let mut assets: HashMap<(PieceKind, Color), Image> = HashMap::new();

    assets.insert(
        (PieceKind::K, Color::White),
        Image::from_uri("file://resources/kw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::R, Color::White),
        Image::from_uri("file://resources/rw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::N, Color::White),
        Image::from_uri("file://resources/nw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::B, Color::White),
        Image::from_uri("file://resources/bw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::Q, Color::White),
        Image::from_uri("file://resources/qw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::P, Color::White),
        Image::from_uri("file://resources/pw.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::K, Color::Black),
        Image::from_uri("file://resources/kb.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::R, Color::Black),
        Image::from_uri("file://resources/rb.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::N, Color::Black),
        Image::from_uri("file://resources/nb.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::B, Color::Black),
        Image::from_uri("file://resources/bb.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::Q, Color::Black),
        Image::from_uri("file://resources/qb.png").fit_to_exact_size(Vec2::new(size, size)),
    );
    assets.insert(
        (PieceKind::P, Color::Black),
        Image::from_uri("file://resources/pb.png").fit_to_exact_size(Vec2::new(size, size)),
    );

    assets
}

pub fn convert_click_to_board_position(
    click_position: Pos2,
    turn: Color,
    square_size: f32,
) -> Position {
    let x = (click_position.x / square_size) as i32;
    let y = (click_position.y / square_size) as i32;

    if turn == Color::White {
        return Position::get_valid_position(7 - x, 7 - y).unwrap();
    }

    Position::get_valid_position(x, y).unwrap()
}

pub fn convert_board_position_to_ui(position: &Position, turn: Color, square_size: f32) -> Pos2 {
    let x: f32;
    let y: f32;

    if turn == Color::White {
        x = (7 - position.x) as f32 * square_size;
        y = (7 - position.y) as f32 * square_size;
    } else {
        x = position.x as f32 * square_size;
        y = position.y as f32 * square_size;
    }
    Pos2::new(x, y)
}
