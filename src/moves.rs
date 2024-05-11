

pub fn get_knight_moves(piece_position: &[usize; 2], opponent_positions: &Vec<[usize; 2]>) -> Vec<(usize, usize)> {
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

    let mut moves: Vec<(usize, usize)> = Vec::new();

    for (move_x, move_y) in knight_moves {
        let new_x = *x as i32 + move_x;
        let new_y = *y as i32 + move_y;
        if (new_x >= 0) & (new_y >= 0) & (new_x <= 7) & (new_y <= 7) {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if !opponent_positions.contains(&[new_x, new_y]) {
                moves.push((new_x, new_y));
            }
        }
    }
    moves
}



mod test_moves {
    use crate::moves::get_knight_moves;

    #[test]
    fn test_get_knight_moves() {
        let piece_position = [1, 6];
        let opponent_positions: Vec<[usize; 2]> = vec![
            [0, 0],
            [2, 6],
            [3, 5],
        ];

        let output = get_knight_moves(&piece_position, &opponent_positions);
        let expected_output: Vec<(usize, usize)> = vec![
            (3, 7),
            (0, 4),
            (2, 4),
        ];
        assert_eq!(expected_output, output);
    }
}
