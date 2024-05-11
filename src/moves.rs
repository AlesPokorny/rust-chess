use crate::helpers::{Direction, Position};

fn get_straight_moves(
    directions: Vec<Direction>,
    piece_position: &Position,
    friendly_positions: &Vec<Position>,
    opponent_positions: &Vec<Position>
) -> Vec<Position> {
    let mut allowed_moves: Vec<Position> = Vec::new();
    let (current_x, current_y) = piece_position.get_x_y_as_int();
    let max_step = [current_x, 7 - current_x, current_y, 7 - current_y].into_iter().max().unwrap();

    for direction in directions {
        for step in 1..max_step {
            if let Some(position) = Position::get_valid_position(
                current_x + step * direction.x,
                current_y + step * direction.y,
            ) {
                if friendly_positions.contains(&position) {
                    break;
                } else if opponent_positions.contains(&position) {
                    allowed_moves.push(position);
                    break;
                } else {
                    allowed_moves.push(position);
                }
            } else {
                break;
            }
        }
    }
    allowed_moves
}

pub fn get_rook_moves(
    piece_position: &Position,
    friendly_positions: &Vec<Position>,
    opponent_positions: &Vec<Position>
) -> Vec<Position> {
    let directions = vec![
        Direction::new(0, 1),
        Direction::new(0, -1),
        Direction::new(-1, 0),
        Direction::new(1, 0),
    ];

    get_straight_moves(directions, piece_position, friendly_positions, opponent_positions)
}

pub fn get_bishop_moves(
    piece_position: &Position,
    friendly_positions: &Vec<Position>,
    opponent_positions: &Vec<Position>
) -> Vec<Position> {
    let directions = vec![
        Direction::new(1, 1),
        Direction::new(1, -1),
        Direction::new(-1, 1),
        Direction::new(-1, -1),
    ];

    get_straight_moves(directions, piece_position, friendly_positions, opponent_positions)
}

pub fn get_queen_moves(
    piece_position: &Position,
    friendly_positions: &Vec<Position>,
    opponent_positions: &Vec<Position>
) -> Vec<Position> {
    let mut moves = get_rook_moves(piece_position, friendly_positions, opponent_positions);
    moves.append(&mut get_bishop_moves(piece_position, friendly_positions, opponent_positions));
    moves
}

pub fn get_knight_moves(piece_position: &Position, friendly_positions: &Vec<Position>) -> Vec<Position> {
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
        if let Some(new_position) = Position::get_valid_position(new_x, new_y){
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
    friendly_positions: &Vec<Position>,
    opponent_positions: &Vec<Position>,
    move_direction: i32,
    en_passant: &Option<Position>
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
        if !friendly_positions.contains(&blocking_position) & !opponent_positions.contains(&blocking_position) {
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

    for pos in capture_moves {
        if let Some(position) = pos {
            if opponent_positions.contains(&position) {
                moves.push(position);
            }
        }
    }
    moves
}

pub fn get_king_moves(position: &Position, friendly_positions: &Vec<Position>) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
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

    for possible_move in possible_moves {
        if let Some(position) = possible_move {
            if !friendly_positions.contains(&position) {
                moves.push(position);
            }
        }
    }

    moves
}


#[cfg(test)]
mod test_moves {
    use crate::moves::{get_knight_moves, get_straight_moves, get_pawn_moves, get_king_moves};
    use crate::helpers::{Direction, Position};

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
        let opponent_positions = vec![
            Position::new(4, 3),
            Position::new(1, 1),
        ];
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

        let output = get_straight_moves(directions, &piece_position, &friendly_positions, &opponent_positions);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_double() {
        let position = Position::new(1, 1);

        let output = get_pawn_moves(
            &position,
            &false,
            &Vec::new(),
            &Vec::new(),
            1,
            &None,
        );
        let expected_output = vec![
            Position::new(1, 2),
            Position::new(1, 3),
        ];

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_single() {
        let position = Position::new(1, 1);

        let output = get_pawn_moves(
            &position,
            &true,
            &Vec::new(),
            &Vec::new(),
            1,
            &None,
        );
        let expected_output = vec![
            Position::new(1, 2),
        ];
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_pawn_moves_blocked_by_friendly() {
        let position = Position::new(1, 1);
        let output = get_pawn_moves(
            &position,
            &false,
            &vec![Position::new(1, 2)],
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
            &vec![Position::new(1, 2)],
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
            &vec![Position::new(1, 2), Position::new(0, 2), Position::new(2, 2)],
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
            &vec![Position::new(0, 1)],
            1,
            &Some(Position::new(0, 2)),
        );
        let expected_output: Vec<Position> = vec![Position::new(1, 2), Position::new(1, 3), Position::new(0, 2)];
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_king_moves() {
        let position = Position::new(4, 4);
        let expected_output = vec![
            Position::new(4, 3),
            Position::new(5, 4),
            Position::new(3, 4),
            Position::new(5, 5),
            Position::new(5, 3),
            Position::new(3, 5),
        ];

        let output = get_king_moves(
            &position,
            &vec![Position::new(1, 2), Position::new(4, 5), Position::new(3, 3)]
        );

        assert_eq!(expected_output, output);
    }

}
