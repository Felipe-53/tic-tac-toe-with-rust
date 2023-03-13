use rand::Rng;
use std::fmt;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Debug)]
enum Player {
    X,
    O,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Cell {
    X,
    O,
    Empty,
}

#[derive(PartialEq)]
enum GameState {
    Win,
    Draw,
    Disputed,
}
struct Game {
    board: [[Cell; 3]; 3],
    turn: Player,
    state: GameState,
}

enum MoveResult {
    Success,
    CellNotEmpty,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[Cell::Empty; 3]; 3],
            turn: Player::X,
            state: GameState::Disputed,
        }
    }

    fn update_state(&mut self) {
        for row in 0..3 {
            if self.board[row][0] == self.board[row][1]
                && self.board[row][1] == self.board[row][2]
                && self.board[row][0] != Cell::Empty
            {
                return self.state = GameState::Win;
            }
        }

        for col in 0..3 {
            if self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
                && self.board[0][col] != Cell::Empty
            {
                return self.state = GameState::Win;
            }
        }

        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != Cell::Empty
        {
            return self.state = GameState::Win;
        }

        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != Cell::Empty
        {
            return self.state = GameState::Win;
        }

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Cell::Empty {
                    return self.state = GameState::Disputed;
                }
            }
        }

        self.state = GameState::Draw;
    }

    fn get_human_move(&self) -> (usize, usize) {
        let selected_position: u8;

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let number: u8 = match input.trim().parse() {
                Ok(input) => input,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };

            if !(1 <= number && number <= 9) {
                println!("Please enter a number between 1 and 9");
                continue;
            }

            selected_position = number;
            break;
        }

        let position = get_position_from_number(selected_position);
        position
    }

    fn get_computer_move(&self) -> (usize, usize) {
        let random_number = rand::thread_rng().gen_range(1..=9);
        let position = get_position_from_number(random_number);
        position
    }

    fn make_move(&mut self, row: usize, col: usize) -> MoveResult {
        if self.board[row][col] != Cell::Empty {
            return MoveResult::CellNotEmpty;
        }

        match self.turn {
            Player::X => self.board[row][col] = Cell::X,
            Player::O => self.board[row][col] = Cell::O,
        }

        self.update_state();
        return MoveResult::Success;
    }

    fn switch_turns(&mut self) {
        match self.turn {
            Player::X => self.turn = Player::O,
            Player::O => self.turn = Player::X,
        }
    }

    fn print(&self) {
        println!();
        println!(
            "{} | {} | {}",
            self.board[0][0], self.board[0][1], self.board[0][2]
        );
        println!(
            "{} | {} | {}",
            self.board[1][0], self.board[1][1], self.board[1][2]
        );
        println!(
            "{} | {} | {}",
            self.board[2][0], self.board[2][1], self.board[2][2]
        );
        println!();
    }
}

fn main() {
    let mut game = Game::new();

    while game.state == GameState::Disputed {
        game.print();

        match game.turn {
            Player::X => {
                println!("Player X's turn: choose a number between 1 and 9:");

                let (row, col) = game.get_human_move();
                match game.make_move(row, col) {
                    MoveResult::Success => (),
                    MoveResult::CellNotEmpty => {
                        println!("Please choose an empty cell");
                        continue;
                    }
                }
            }

            Player::O => {
                println!("Player O's turn");

                loop {
                    let (row, col) = game.get_computer_move();
                    match game.make_move(row, col) {
                        MoveResult::Success => break,
                        MoveResult::CellNotEmpty => {
                            continue;
                        }
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        }

        game.switch_turns();
    }

    game.print();

    if game.state == GameState::Draw {
        println!("It's a draw!");
        return;
    }

    game.switch_turns();
    println!("Player {:?} won!", game.turn);
}

fn get_position_from_number(i: u8) -> (usize, usize) {
    match i {
        1 => (2, 0),
        2 => (2, 1),
        3 => (2, 2),
        4 => (1, 0),
        5 => (1, 1),
        6 => (1, 2),
        7 => (0, 0),
        8 => (0, 1),
        9 => (0, 2),
        _ => (0, 0),
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::Empty => write!(f, "_"),
        }
    }
}
