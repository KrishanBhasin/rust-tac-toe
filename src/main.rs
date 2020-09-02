use std::io;
use std::num::ParseIntError;

mod tic_tac_toe_board;

use crate::tic_tac_toe_board::TicTacToeBoard;


fn main() {
    let mut board = TicTacToeBoard::create_new();

    while board.check_for_winner().is_none() {
        board = do_turn(board);
    }

}


fn do_turn(board: TicTacToeBoard) -> TicTacToeBoard {
    
    let mut user_input = String::new();
    println!("Please enter which tile you wish to place your piece on: [0-8]");
    io::stdin().read_line(&mut user_input).unwrap();

    let chosen_location:Result<u8, ParseIntError> = user_input.trim().parse();
    let chosen_location: usize = match chosen_location {
        Ok(loc) => loc as usize,
        Err(_err) => panic!("Looks like you didn't put a number in!"),
    };


    board.place_piece(chosen_location)

}

