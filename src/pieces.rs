

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
    P,
    R,
    N,
    B,
    K,
    Q,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub position: [usize; 2],
    pub points: i32,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind, position: [usize; 2]) -> Piece {
        let points = match kind {
            PieceKind::P => 1,
            PieceKind::R => 5,
            PieceKind::N => 3,
            PieceKind::B => 3,
            PieceKind::Q => 9,
            PieceKind::K => 0,
        };
        Piece {
            color,
            kind,
            position,
            points,
        }
    }

    fn move_piece(mut self, new_position: [usize; 2]) {
        self.position = new_position
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = match self.kind {
            PieceKind::R => 'R',
            PieceKind::N => 'N',
            PieceKind::B => 'B',
            PieceKind::Q => 'Q',
            PieceKind::K => 'K',
            PieceKind::P => 'P',
        };
        
        if matches!(self.color, Color::Black) {
            output = output.to_ascii_lowercase();
        }

        write!(f, "{}", output)
    }
}
