use lib::{
    clear_terminal, controls, generate_board, reveal_cell, show_board, Cell, CellState, GameState,
};
use std::io::{self, Write};
use std::num::ParseIntError;

fn check_if_win(board: &mut Vec<Vec<Cell>>) -> bool {
    for row in board {
        for cell in row {
            if cell.value != 9 && cell.state != CellState::Revealed {
                return false;
            }
        }
    }
    true
}

fn choose_difficulty() -> Result<usize, ParseIntError> {
    let mut choose = String::new();

    clear_terminal();

    println!("Choose the difficulty level:");
    println!("1. 8x8 board, 10 mines");
    println!("2. 16x16 board, 40 mines");
    println!("3. 30x16 board, 99 mines");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");

    choose.trim().parse::<usize>()
}

fn main() {
    let mut game_state = GameState::InProgress;
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    let mut board: Vec<Vec<Cell>>;
    let mut mines_generated = false;

    let mines: usize;

    loop {
        match choose_difficulty() {
            Ok(1) => {
                board = generate_board(8, 8);
                mines = 10;
                break;
            }
            Ok(2) => {
                board = generate_board(16, 16);
                mines = 40;
                break;
            }
            Ok(3) => {
                board = generate_board(30, 16);
                mines = 99;
                break;
            }
            _ => {
                eprintln!("Invalid choice. Please provide a valid option");
            }
        }
    }

    show_board(&board, pos_x, pos_y, mines);

    while game_state == GameState::InProgress {
        controls(
            &mut board,
            &mut pos_x,
            &mut pos_y,
            &mut game_state,
            &mut mines_generated,
            mines,
        );

        if check_if_win(&mut board) {
            game_state = GameState::Won;
        }

        show_board(&board, pos_x, pos_y, mines);
    }

    if game_state == GameState::Won {
        println!("You won!");
    }
    if game_state == GameState::Lost {
        reveal_cell(&mut board, pos_x, pos_y);
        show_board(&board, pos_x, pos_y, mines);
        println!("Boom! You loose");
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
