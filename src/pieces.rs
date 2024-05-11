

pub enum Color {
    White,
    Black,
}

pub enum PieceKind {
    P,
    R,
    N,
    B,
    K,
    Q,
}
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
    pub position: [i8; 2],
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind, position: [i8; 2]) -> Piece {
        Piece {
            color,
            kind,
            position,
        }
    }

    fn move_piece(mut self, new_position: [i8; 2]) {
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
