use crate::generation::generate_board;
use crate::other::{Cell, GameState};
use crate::visuals::clear_terminal;
use std::io::{self, Write};
use std::num::ParseIntError;

fn choose_difficulty() -> Result<usize, ParseIntError> {
    let mut choose = String::new();

    clear_terminal();

    println!("Choose the difficulty level:");
    println!("1. 8x8 board, 10 mines");
    println!("2. 16x16 board, 40 mines");
    println!("3. 30x16 board, 99 mines");
    println!("4. Exit game");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");

    choose.trim().parse::<usize>()
}

pub fn init_game(
    game_state: &mut GameState,
    board: &mut Vec<Vec<Cell>>,
    mines_generated: &mut bool,
    mines: &mut usize,
) {
    *mines_generated = false;

    loop {
        match choose_difficulty() {
            Ok(1) => {
                *board = generate_board(8, 8);
                *mines = 10;
                break;
            }
            Ok(2) => {
                *board = generate_board(16, 16);
                *mines = 40;
                break;
            }
            Ok(3) => {
                *board = generate_board(30, 16);
                *mines = 99;
                break;
            }
            Ok(4) => {
                *game_state = GameState::Ended;
                *board = generate_board(0, 0);
                *mines = 0;
                break;
            }
            _ => eprintln!("Invalid choice. Please provide a valid option"),
        };
    }
}

pub fn next_round() -> GameState {
    let mut choose = String::new();

    println!("Choose what to do next:");
    println!("1. Play next round");
    println!("2. Exit game");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");

    loop {
        match choose.trim().parse::<usize>() {
            Ok(1) => return GameState::InProgress,
            Ok(2) => return GameState::Ended,
            _ => eprintln!("Invalid choice. Please provide a valid option"),
        };
    }
}
