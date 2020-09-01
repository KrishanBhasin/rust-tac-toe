use std::fmt;

fn main() {
    let mut board = TicTacToeBoard{
        // Ugly hack to initialise the array.
        // squares: [SquareState::Empty; 9] // this would be much nicer
        squares: [SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,SquareState::Empty,],
        last_play: Option::None
    };

    println!("{}", board);
    println!("========");
    board = board.place_piece(SquareState::X, 0);
    println!("{}", board);
    println!("========");
    board = board.place_piece(SquareState::O, 2);
    println!("{:?}", board);
}


#[derive(Debug, Copy, Clone)]
struct TicTacToeBoard {
    squares: [SquareState; 9],
    last_play: Option<LastPlay>
}
#[derive(Debug, Copy, Clone)]
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
    fn place_piece(mut self, piece: SquareState, location: usize) -> TicTacToeBoard {
        self.last_play = Some(LastPlay{
            piece: piece.clone(),
            location: location
        });
        self.squares[location] = piece.clone();
        return self;
    }
}


#[derive(Debug, Copy, Clone)]
enum SquareState {
    X,
    O,
    Empty
}
impl fmt::Display for SquareState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           SquareState::X => write!(f, "X"),
           SquareState::O => write!(f, "O"),
           SquareState::Empty => write!(f, " "),
       }
    }
}