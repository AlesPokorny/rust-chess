use crate::positions::Position;

fn is_position_on_board(position: &Position) -> bool {
    if (position.x >= 0) & (position.x <= 7) & (position.y >= 0) & (position.y <= 7) {
        return true
    }
    false
}

pub fn get_knight_moves(piece_position: &[usize; 2], friendly_positions: &Vec<Position>) -> Vec<Position> {
    let [x, y] = piece_position;
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
        let new_x = *x as i32 + move_x;
        let new_y = *y as i32 + move_y;
        let new_position = Position::new(new_x as usize, new_y as usize);
        if is_position_on_board(&new_position) {
            if !friendly_positions.contains(&new_position) {
                moves.push(new_position);
            }
        }
    }
    moves
}



mod test_moves {
    use crate::moves::get_knight_moves;
    use crate::positions::Position;

    #[test]
    fn test_get_knight_moves() {
        let piece_position = [1, 6];
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
}
