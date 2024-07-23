use crate::helpers::Position;
use crate::pieces::{Color, PieceKind};
use eframe::egui::{
    Color32, FontId, Image, Pos2, Rect, RichText, Rounding, Shape, Slider, Stroke, TextStyle, Ui,
    Vec2,
};
use eframe::epaint::FontFamily;

use fnv::FnvHashMap;

pub fn make_square(rect: Rect, color: Color32, fill: bool) -> Shape {
    if fill {
        return Shape::rect_filled(rect, Rounding::ZERO, color);
    }

    Shape::rect_stroke(rect, Rounding::ZERO, Stroke::new(3.0, color))
}

pub fn init_assets<'a>(size: f32) -> FnvHashMap<(PieceKind, Color), Image<'a>> {
    let mut assets: FnvHashMap<(PieceKind, Color), Image> =
        FnvHashMap::with_capacity_and_hasher(12, Default::default());

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
    player_color: Color,
    square_size: f32,
) -> Position {
    let x = (click_position.x / square_size) as i32;
    let y = (click_position.y / square_size) as i32;

    if player_color == Color::White {
        return Position::get_valid_position(7 - x, 7 - y).unwrap();
    }

    Position::get_valid_position(x, y).unwrap()
}

pub fn convert_board_position_to_ui(
    position: &Position,
    player_color: Color,
    square_size: f32,
) -> Pos2 {
    let x: f32;
    let y: f32;

    if player_color == Color::White {
        x = (7 - position.x) as f32 * square_size;
        y = (7 - position.y) as f32 * square_size;
    } else {
        x = position.x as f32 * square_size;
        y = position.y as f32 * square_size;
    }
    Pos2::new(x, y)
}

pub fn draw_color_sliders(colors: &mut [f32; 3], ui: &mut Ui, label: &str, font_size: f32) {
    ui.style_mut().text_styles.insert(
        TextStyle::Button,
        FontId::new(font_size, FontFamily::Proportional),
    );
    ui.label(RichText::new(label).size(font_size));

    ui.style_mut().spacing.slider_width = ui.max_rect().width() / 2.;
    ui.add(
        Slider::new(&mut colors[0], 0.0..=255.0)
            .text("R")
            .max_decimals(0),
    );
    ui.add(
        Slider::new(&mut colors[1], 0.0..=255.0)
            .text("G")
            .max_decimals(0),
    );
    ui.add(
        Slider::new(&mut colors[2], 0.0..=255.0)
            .text("B")
            .max_decimals(0),
    );
    ui.layout().prefer_right_to_left();

    let right_top = ui.max_rect().right_top();
    let rect_width = ui.max_rect().width() / 5.;
    let left_top = Pos2::new(right_top.x - rect_width, right_top.y);
    let rect = Rect::from_min_size(left_top, Vec2::new(rect_width, ui.max_rect().height()));
    let square = make_square(
        rect,
        Color32::from_rgb(colors[0] as u8, colors[1] as u8, colors[2] as u8),
        true,
    );
    ui.painter().add(square);
}
