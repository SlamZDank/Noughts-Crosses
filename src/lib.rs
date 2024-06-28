mod winning_patters;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Gamers {
    X,
    O,
}

pub fn get_new_board() -> Vec<Option<Gamers>> {
    vec![None; 9]
}

pub fn put_marker_on_board(
    board: &mut Vec<Option<Gamers>>,
    position: usize,
    marker: &Gamers,
) -> bool {
    if position > 9 || board[position].is_some() {
        return false;
    }
    board[position] = Some(marker.clone());
    true
}

fn analyse_board(board: &Vec<Option<Gamers>>, marker: Gamers) -> Vec<u8> {
    let mut board_generated_usize: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..9 {
        if let Some(marked) = &board[i] {
            if *marked == marker {
                board_generated_usize[i] = 1;
            }
        }
    }

    board_generated_usize
}

fn binary_and_operation(board_a: &Vec<u8>, board_b: Vec<u8>) -> Vec<u8> {
    let mut resulting_board: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    for idx in 0..9 {
        if board_a[idx] == 1 && board_b[idx] == 1 {
            resulting_board[idx] = 1;
        }
    }

    resulting_board
}

pub fn verify_winner(board: &Vec<Option<Gamers>>) -> Option<Gamers> {
    let generated_x_board = analyse_board(board, Gamers::X);
    let generated_o_board = analyse_board(board, Gamers::O);

    for winning_tables in winning_patters::WINNNING_PATTERNS {
        if binary_and_operation(&generated_x_board, winning_tables.to_vec())
            == winning_tables.to_vec()
        {
            return Some(Gamers::X);
        }
        if binary_and_operation(&generated_o_board, winning_tables.to_vec())
            == winning_tables.to_vec()
        {
            return Some(Gamers::O);
        }
    }
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn verify_winner() {}
}
