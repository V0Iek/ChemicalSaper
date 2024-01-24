use crate::other::{Cell, CellState};
use crossterm::execute;
use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use std::io::{self, Write};

pub fn clear_terminal() {
    execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");

    print!("\x1B[H");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn color_value(value: i32) -> StyledContent<&'static str> {
    match value {
        1 => "1".blue(),
        2 => "2".green(),
        3 => "3".red(),
        4 => "4".dark_blue(),
        5 => "5".dark_red(),
        6 => "6".cyan(),
        7 => "7".yellow(),
        8 => "8".white(),
        _ => {
            println!("Error");
            "Invalid".on_red()
        }
    }
}

pub fn show_board(board: Vec<Vec<Cell>>, pos_x: usize, pos_y: usize) {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    clear_terminal();

    let width = board[0].len() - 1;
    let height = board.len() - 1;

    for i in 0..=height {
        for j in 0..=width {
            if pos_y == i && pos_x == j {
                if board[i][j].state == CellState::Revealed {
                    if board[i][j].value == 0 {
                        print!("{} ", "O".dark_green());
                    } else {
                        print!("{} ", board[i][j].value.to_string().dark_green());
                    }
                } else if board[i][j].state == CellState::Flagged {
                    print!("{} ", "P".dark_green());
                } else {
                    print!("{} ", "#".dark_green());
                }
            } else {
                if board[i][j].state == CellState::Revealed {
                    if board[i][j].value == 0 {
                        print!("  ");
                    } else {
                        print!("{} ", color_value(board[i][j].value));
                    }
                } else if board[i][j].state == CellState::Flagged {
                    print!("P ");
                } else {
                    print!("# ");
                }
            }
        }
        print!("\n");
    }
    println!("Cursor position:");
    println!("X: {}", pos_x);
    println!("Y: {}", pos_y);

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
}

pub fn reveal_cell(board: &mut Vec<Vec<Cell>>, mut x: usize, mut y: usize) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    if x > width || y > height || board[y][x].state != CellState::Hidden || board[y][x].value == 9 {
        return;
    }

    if board[y][x].state == CellState::Hidden {
        board[y][x].state = CellState::Revealed;
    }

    for i in 0..=2 {
        for j in 0..=2 {
            if x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                reveal_cell(board, x + i, y + j);
            }
        }
    }
}
