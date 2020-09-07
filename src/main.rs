mod tic_tac_toe_board;

use crate::tic_tac_toe_board::TicTacToeBoard;
use crate::tic_tac_toe_board::Player;


fn main() {
    let mut board = TicTacToeBoard::create_new();

    while board.check_for_winner().is_none() {
        board = do_turn(board);
    }

}


fn do_turn(board: TicTacToeBoard) -> TicTacToeBoard {
    let chosen_location: usize = board.select_piece_placement(Player::Human);

    board.place_piece(chosen_location)
}

