use crate::board::Board;
use crate::helpers::{Direction, Position};
use crate::pieces::{Color, Piece, PieceKind};

// TODO: This needs to change to constants so it can be fastaaaah
fn bishop_directions() -> Vec<Direction> {
    vec![
        Direction::new(1, 1),
        Direction::new(1, -1),
        Direction::new(-1, 1),
        Direction::new(-1, -1),
    ]
}

fn rook_directions() -> Vec<Direction> {
    vec![
        Direction::new(0, 1),
        Direction::new(0, -1),
        Direction::new(-1, 0),
        Direction::new(1, 0),
    ]
}

fn get_straight_moves(
    directions: Vec<Direction>,
    piece_position: &Position,
    friendly_positions: &[Position],
    opponent_positions: &[Position],
) -> Vec<Vec<Position>> {
    let mut allowed_moves: Vec<Vec<Position>> = Vec::new();
    let (current_x, current_y) = piece_position.get_x_y_as_int();
    let max_step = [current_x, 7 - current_x, current_y, 7 - current_y]
        .into_iter()
        .max()
        .unwrap();

    for direction in directions.iter() {
        let mut moves_in_one_direction: Vec<Position> = Vec::new();
        for step in 1..max_step {
            if let Some(position) = Position::get_valid_position(
                current_x + step * direction.x,
                current_y + step * direction.y,
            ) {
                if friendly_positions.contains(&position) {
                    break;
                } else if opponent_positions.contains(&position) {
                    moves_in_one_direction.push(position);
                    break;
                } else {
                    moves_in_one_direction.push(position);
                }
            } else {
                break;
            }
        }
        allowed_moves.push(moves_in_one_direction);
    }
    allowed_moves
}

pub fn get_rook_moves(
    piece_position: &Position,
    friendly_positions: &[Position],
    opponent_positions: &[Position],
) -> Vec<Position> {
    get_straight_moves(
        rook_directions(),
        piece_position,
        friendly_positions,
        opponent_positions,
    )
    .into_iter()
    .flatten()
    .collect()
}

pub fn get_bishop_moves(
    piece_position: &Position,
    friendly_positions: &[Position],
    opponent_positions: &[Position],
) -> Vec<Position> {
    get_straight_moves(
        bishop_directions(),
        piece_position,
        friendly_positions,
        opponent_positions,
    )
    .into_iter()
    .flatten()
    .collect()
}

pub fn get_queen_moves(
    piece_position: &Position,
    friendly_positions: &[Position],
    opponent_positions: &[Position],
) -> Vec<Position> {
    let mut moves = get_rook_moves(piece_position, friendly_positions, opponent_positions);
    moves.append(&mut get_bishop_moves(
        piece_position,
        friendly_positions,
        opponent_positions,
    ));
    moves
}

pub fn get_knight_moves(
    piece_position: &Position,
    friendly_positions: &[Position],
) -> Vec<Position> {
    let x = piece_position.x;
    let y = piece_position.y;
    let knight_moves: [(i32, i32); 8] = [
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (-1, -2),
        (1, -2),
        (-1, 2),
        (1, 2),
    ];
    let mut moves: Vec<Position> = Vec::new();

    for (move_x, move_y) in knight_moves {
        let new_x = x as i32 + move_x;
        let new_y = y as i32 + move_y;
        if let Some(new_position) = Position::get_valid_position(new_x, new_y) {
            if !friendly_positions.contains(&new_position) {
                moves.push(new_position);
            }
        }
    }
    moves
}

pub fn get_pawn_moves(
    position: &Position,
    has_moved: &bool,
    friendly_positions: &[Position],
    opponent_positions: &[Position],
    move_direction: i32,
    en_passant: &Option<Position>,
) -> Vec<Position> {
    // Fuck Pawns
    let mut moves: Vec<Position> = Vec::new();
    let (x, y) = position.get_x_y_as_int();
    let mut opponent_positions = opponent_positions.to_vec();

    if let Some(en_passant_position) = en_passant {
        opponent_positions.push(*en_passant_position)
    }

    let mut forward_moves: Vec<Position> = Vec::new();

    if let Some(position) = Position::get_valid_position(x, y + move_direction) {
        forward_moves.push(position);
    }

    if !has_moved {
        let blocking_position = Position::get_valid_position(x, y + move_direction).unwrap();
        if !friendly_positions.contains(&blocking_position)
            & !opponent_positions.contains(&blocking_position)
        {
            forward_moves.push(Position::get_valid_position(x, y + 2 * move_direction).unwrap());
        }
    }

    for position in forward_moves {
        if !friendly_positions.contains(&position) & !opponent_positions.contains(&position) {
            moves.push(position);
        }
    }

    let capture_moves = [
        Position::get_valid_position(x - 1, y + move_direction),
        Position::get_valid_position(x + 1, y + move_direction),
    ];

    for position in capture_moves.into_iter().flatten() {
        if opponent_positions.contains(&position) {
            moves.push(position);
        }
    }
    moves
}

fn get_castling_move(board: &Board, short: bool) -> Option<Position> {
    let row = if board.turn == Color::White { 0 } else { 7 };
    let col: Vec<usize>;
    let target_col: usize;

    if short {
        col = vec![1, 2];
        target_col = 1;
    } else {
        col = vec![4, 5];
        if let Some(_) = board.get_piece_from_position(&Position::new(6, row)) {
            return None
        }
        target_col = 5;
    }

    for i in 0..=1_usize {
        let position = Position::new(col[i], row);
        if let Some(_) = board.get_piece_from_position(&position) {
            return None
        }
        if is_field_in_check(position, board) {
            return None
        }
    };

    Some(Position::new(target_col, row))
}

pub fn get_king_moves(position: &Position, board: &Board, friendly_positions: &[Position]) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();

    let [can_castle_short, can_castle_long] = board.castling[&board.turn];
    
    if can_castle_short {
        if let Some(castling_move) = get_castling_move(board, true) {
            moves.push(castling_move);
        }
    }
    if can_castle_long {
        if let Some(castling_move) = get_castling_move(board, false) {
            moves.push(castling_move);
        }
    }

    let (x, y) = position.get_x_y_as_int();

    let possible_moves = [
        Position::get_valid_position(x, y + 1),
        Position::get_valid_position(x, y - 1),
        Position::get_valid_position(x + 1, y),
        Position::get_valid_position(x - 1, y),
        Position::get_valid_position(x + 1, y + 1),
        Position::get_valid_position(x + 1, y - 1),
        Position::get_valid_position(x - 1, y + 1),
        Position::get_valid_position(x - 1, y - 1),
    ];

    for position in possible_moves.into_iter().flatten() {
        if !friendly_positions.contains(&position) {
            moves.push(position);
        }
    }

    moves
}

pub fn is_field_in_check(field_position: Position, board: &Board) -> bool {
    let all_positions = board.get_all_positions();
    let friendly_positions = &all_positions[(board.turn == Color::Black) as usize];
    let opponent_positions = &all_positions[(board.turn != Color::Black) as usize];

    let knight_moves = get_knight_moves(&field_position, friendly_positions);
    for knight_move in knight_moves {
        if let Some(piece) = board.get_piece_from_position(&knight_move) {
            if (piece.color != board.turn) & (piece.kind == PieceKind::N) {
                return true;
            }
        }
    }

    // rooks, queen - the straight boiiiz
    let rook_moves = get_straight_moves(
        rook_directions(),
        &field_position,
        friendly_positions,
        opponent_positions,
    );
    let last_rook_moves = rook_moves
        .iter()
        .map(|positions_in_direction| positions_in_direction.last());
    for last_rook_move in last_rook_moves.into_iter().flatten() {
        if let Some(piece) = board.get_piece_from_position(last_rook_move) {
            if (piece.color != board.turn)
                & ((piece.kind == PieceKind::R) | (piece.kind == PieceKind::Q))
            {
                return true;
            }
        }
    }

    // bishops, queen - the diagonal boiiz
    let bishop_moves = get_straight_moves(
        bishop_directions(),
        &field_position,
        friendly_positions,
        opponent_positions,
    );
    let last_bishop_moves: Vec<Option<&Position>> = bishop_moves
        .iter()
        .map(|positions_in_direction| positions_in_direction.last())
        .collect();
    for last_bishop_move in last_bishop_moves.into_iter().flatten() {
        if let Some(piece) = board.get_piece_from_position(last_bishop_move) {
            if (piece.color != board.turn)
                & ((piece.kind == PieceKind::B) | (piece.kind == PieceKind::Q))
            {
                return true;
            }
        }
    }

    // p(r)awns
    let attack_direction = if board.turn == Color::White { 1 } else { -1 };
    for direction in [
        Direction::new(-1, attack_direction),
        Direction::new(1, attack_direction),
    ] {
        if let Some(position_to_check) = Position::get_valid_position(
            field_position.x as i32 + direction.x,
            field_position.y as i32 + direction.y,
        ) {
            if let Some(piece) = board.get_piece_from_position(&position_to_check) {
                if (piece.color != board.turn) & (piece.kind == PieceKind::P) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn filter_check_moves(
    from_position: Position,
    to_positions: Vec<Position>,
    board: &Board,
    piece: Piece,
) -> Vec<Position> {
    // this method needs to be cleaned up
    // also it is missing en passant capture checks
    let mut filtered_moves: Vec<Position> = Vec::new();

    for to_position in to_positions {
        let mut temp_board = board.clone();
        let moved_piece = match &mut temp_board.board[from_position.x][from_position.y] {
            Some(moved_piece) => moved_piece,
            None => panic!("Oops, you done goofed"),
        };
        moved_piece.move_piece(to_position);
        temp_board.move_piece(&from_position, &to_position);
        let king_position = if piece.kind == PieceKind::K {
            to_position
        } else {
            temp_board.king_positions[&board.turn]
        };

        let is_check = is_field_in_check(king_position, &temp_board);

        if !is_check {
            filtered_moves.push(to_position);
        }
    }

    filtered_moves
}

pub fn get_rook_old_and_new_castling_positions(king_new_position: &Position) -> (Position, Position) {
    let old_x_position: usize;
    let new_x_position: usize;
    if king_new_position.x == 1 {
        old_x_position = 0;
        new_x_position = 2;
    } else {
        old_x_position = 7;
        new_x_position = 4;
    }
    let old_rook_position = Position::new(old_x_position, king_new_position.y);
    let new_rook_position = Position::new(new_x_position, king_new_position.y);

    (old_rook_position, new_rook_position)
}

#[cfg(test)]
mod test_moves {
    use crate::helpers::{Direction, Position};
    use crate::moves::{get_king_moves, get_knight_moves, get_pawn_moves, get_straight_moves};

    #[test]
    fn test_get_knight_moves() {
        let piece_position = Position::new(1, 6);
        let friendly_positions: Vec<Position> = vec![
            Position::new(0, 0),
            Position::new(2, 6),
            Position::new(3, 5),
        ];

        let output = get_knight_moves(&piece_position, &friendly_positions);
        let expected_output: Vec<Position> = vec![
            Position::new(3, 7),
            Position::new(0, 4),
            Position::new(2, 4),
        ];
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_straight_moves() {
        let piece_position = Position::new(4, 4);
        let directions = vec![
            Direction::new(-1, 0),
            Direction::new(1, 0),
            Direction::new(0, 1),
            Direction::new(0, -1),
            Direction::new(-1, -1),
        ];
        let friendly_positions = vec![
            Position::new(4, 5),
            Position::new(2, 4),
            Position::new(6, 2),
        ];
        let opponent_positions = vec![Position::new(4, 3), Position::new(1, 1)];
        let expected_output = vec![
            Position::new(3, 4),
            Position::new(5, 4),
            Position::new(6, 4),
            Position::new(7, 4),
            Position::new(4, 3),
            Position::new(3, 3),
            Position::new(2, 2),
            Position::new(1, 1),
        ];

        let output: Vec<Position> = get_straight_moves(
            directions,
            &piece_position,
            &friendly_positions,
            &opponent_positions,
        )
        .into_iter()
        .flatten()
        .collect();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_double() {
        let position = Position::new(1, 1);

        let output = get_pawn_moves(&position, &false, &Vec::new(), &Vec::new(), 1, &None);
        let expected_output = vec![Position::new(1, 2), Position::new(1, 3)];

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_single() {
        let position = Position::new(1, 1);

        let output = get_pawn_moves(&position, &true, &Vec::new(), &Vec::new(), 1, &None);
        let expected_output = vec![Position::new(1, 2)];
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_blocked_by_friendly() {
        let position = Position::new(1, 1);
        let output = get_pawn_moves(
            &position,
            &false,
            &[Position::new(1, 2)],
            &Vec::new(),
            1,
            &None,
        );
        let expected_output: Vec<Position> = Vec::new();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_blocked_by_opponent() {
        let position = Position::new(1, 1);
        let output = get_pawn_moves(
            &position,
            &false,
            &Vec::new(),
            &[Position::new(1, 2)],
            1,
            &None,
        );
        let expected_output: Vec<Position> = Vec::new();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_capture() {
        let position = Position::new(1, 1);
        let output = get_pawn_moves(
            &position,
            &false,
            &Vec::new(),
            &[
                Position::new(1, 2),
                Position::new(0, 2),
                Position::new(2, 2),
            ],
            1,
            &None,
        );
        let expected_output: Vec<Position> = vec![Position::new(0, 2), Position::new(2, 2)];
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_en_passant() {
        let position = Position::new(1, 1);
        let output = get_pawn_moves(
            &position,
            &false,
            &Vec::new(),
            &[Position::new(0, 1)],
            1,
            &Some(Position::new(0, 2)),
        );
        let expected_output: Vec<Position> = vec![
            Position::new(1, 2),
            Position::new(1, 3),
            Position::new(0, 2),
        ];
        assert_eq!(expected_output, output);
    }
}
