use noughts_crosses::{self, get_new_board, put_marker_on_board, verify_winner, Gamers};
use std::io;

fn is_full(board: &Vec<Option<Gamers>>) -> bool {
    for entry in board {
        if Option::is_none(entry) {
            return false;
        }
    }
    true
}

fn print_board(board: &Vec<Option<Gamers>>) {
    let mut text = String::new();
    let mut i = 0;
    for entry in board {
        match entry {
            Some(Gamers::X) => text += "| X |",
            Some(Gamers::O) => text += "| O |",
            None => text += "|   |",
        }
        i += 1;
        if i == 3 {
            text += "\n";
            i = 0;
        }
    }
    println!("\n{text}\n");
}

pub fn main() {
    let mut turn = Gamers::X;
    let mut board = get_new_board();
    print!("{}[2J", 27 as char);

    print_board(&board);
    while verify_winner(&board).is_none() && !is_full(&board) {
        loop {
            let mut choice = String::new();
            println!("{}'s turn", if turn == Gamers::X { "X" } else { "O" });
            println!("Pick a position: ");
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().parse::<usize>().unwrap();
            if put_marker_on_board(&mut board, choice - 1, &turn) {
                turn = {
                    if turn == Gamers::X {
                        Gamers::O
                    } else {
                        Gamers::X
                    }
                };
                break;
            }
        }
        //println!("{:?}, {:?}", verify_winner(&board), is_full(&board));
        print!("{}[2J", 27 as char);
        print_board(&board);
    }

    match verify_winner(&board) {
        Some(Gamers::X) => println!("X wins!"),
        Some(Gamers::O) => println!("O wins!"),
        None => println!("Draw!"),
    }
}
