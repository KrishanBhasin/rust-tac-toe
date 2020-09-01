use std::fmt;
use std::io;
use std::num::ParseIntError;

use rand;

fn main() {
    let mut board = TicTacToeBoard::create_new();
    println!("{}", board);
    println!("========");

    let mut user_input = String::new();
    println!("Please enter which tile you wish to place your piece on: [0-8]");
    io::stdin().read_line(&mut user_input).unwrap();

    let chosen_location:Result<u8, ParseIntError> = user_input.trim().parse();
    let chosen_location: usize = match chosen_location {
        Ok(loc) => loc as usize,
        Err(_err) => panic!("Looks like you didn't put a number in!"),
    };


    board = board.place_piece(chosen_location);
    println!("{}", board);
    println!("========");

}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct TicTacToeBoard {
    squares: [Square; 9],
    last_play: Option<LastPlay>,
    next_turn: SquareState
}

impl TicTacToeBoard {
    fn create_new() -> TicTacToeBoard {
        TicTacToeBoard{
            squares: [Square{state:None};9],
            last_play: Option::None,
            next_turn: if rand::random() {
                    SquareState::X
                } else {
                    SquareState::O
                },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LastPlay {
    location: usize,
    piece: SquareState
}

impl fmt::Display for TicTacToeBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = format!("{:^5}+{:^5}+{:^5}\n", "---", "---", "---");
        // Hacky workaround due to not fully understanding https://doc.rust-lang.org/std/fmt/#fillalignment (yet)
        // "Note that alignment may not be implemented by some types. In particular, it is not generally implemented for the Debug trait.
        // A good way to ensure padding is applied is to format your input, then pad this resulting string to obtain your output"
        write!(
            f,
            "
            {:^5}|{:^5}|{:^5}\n
            {}
            {:^5}|{:^5}|{:^5}\n
            {}
            {:^5}|{:^5}|{:^5}\n
            \n",
            format!("{}", self.squares[0]), format!("{}", self.squares[1]), format!("{}", self.squares[2]),
            separator,
            format!("{}", self.squares[3]), format!("{}", self.squares[4]), format!("{}", self.squares[5]),
            separator,
            format!("{}", self.squares[6]), format!("{}", self.squares[7]), format!("{}", self.squares[8])
        )
    }
}


impl TicTacToeBoard{
    fn place_piece(mut self, location: usize) -> TicTacToeBoard {
        self.last_play = Some(LastPlay{
            piece: self.next_turn.clone(),
            location
        });
        self.squares[location] = Square{state:Some(self.next_turn.clone())};
        self.next_turn = match self.next_turn {
            SquareState::X => SquareState::O,
            SquareState::O => SquareState::X,
        };
        if self.check_for_winner() {
            panic!("SOMEBODY WON THE GAME!!!!11111")
        }
        return self;
    }

    fn check_for_winner(self) -> bool {
        // Horrible 8-way check for winning combinations
        // Checking `is_some()` followed by equality feels questionable

        // Top row
        if self.squares[0].state.is_some() && self.squares[0] == self.squares[1] && self.squares[1] == self.squares[2] {
            return true;
        } else
        // Middle row
        if self.squares[3].state.is_some() && self.squares[3] == self.squares[4] && self.squares[4] == self.squares[5] {
            return true
        } else
        // Bottom row
        if self.squares[6].state.is_some() && self.squares[6] == self.squares[7] && self.squares[7] == self.squares[8] {
            return true
        } else
        // Left column
        if self.squares[0].state.is_some() && self.squares[0] == self.squares[3] && self.squares[3] == self.squares[6] {
            return true
        } else
        // Middle column
        if self.squares[1].state.is_some() && self.squares[1] == self.squares[4] && self.squares[4] == self.squares[7] {
            return true
        } else
        // Right column
        if self.squares[2].state.is_some() && self.squares[2] == self.squares[5] && self.squares[5] == self.squares[8] {
            return true
        } else
        // Top left to bottom right
        if self.squares[0].state.is_some() && self.squares[0] == self.squares[4] && self.squares[4] == self.squares[8] {
            return true
        } else
        // Top right to bottom left
        if self.squares[2].state.is_some() && self.squares[2] == self.squares[4] && self.squares[4] == self.squares[6] {
            return true
        } else {
            return false
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Square {
    state: Option<SquareState>
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            None => write!(f," "),
            Some(s) => write!(f,"{}",s)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SquareState {
    X,
    O,
}

impl fmt::Display for SquareState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           SquareState::X => write!(f, "X"),
           SquareState::O => write!(f, "O")
       }
    }
}


