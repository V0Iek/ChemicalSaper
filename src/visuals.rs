use crate::enums::CellState;
use crate::structs::Cell;
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

fn count_flags(board: &Vec<Vec<Cell>>) -> usize {
    let mut flags: usize = 0;

    for row in board {
        for cell in row {
            if cell.state == CellState::Flagged {
                flags += 1;
            }
        }
    }

    flags
}

pub fn show_board(board: &Vec<Vec<Cell>>, pos_x: usize, pos_y: usize, mines: usize) {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    clear_terminal();

    let width = board[0].len() - 1;
    let height = board.len() - 1;

    for i in 0..=height {
        for j in 0..=width {
            if pos_y == i && pos_x == j {
                if board[i][j].state == CellState::Revealed {
                    if board[i][j].value == 0 {
                        print!("{} ", " ".on_grey());
                    } else {
                        print!("{} ", color_value(board[i][j].value).to_string().on_grey());
                    }
                } else if board[i][j].state == CellState::Flagged {
                    print!("{} ", "P".on_grey());
                } else {
                    print!("{} ", "#".on_grey());
                }
            } else if board[i][j].state == CellState::Revealed {
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

        if i == 0 {
            println!("    Mines:");
        } else if i == 1 {
            println!("    {}/{}", count_flags(board), mines);
        } else {
            println!();
        }
    }

    println!("Cursor position:");
    println!("X: {}", pos_x);
    println!("Y: {}", pos_y);

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
}

pub fn reveal_cell(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    if x > width || y > height || board[y][x].value == 9 {
        return;
    }

    if board[y][x].state == CellState::Hidden {
        board[y][x].state = CellState::Revealed;
    } else {
        return;
    }

    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x as isize + j as isize;
            let new_y = y as isize + i as isize;

            if new_x >= 0 && new_y >= 0 && new_x <= width as isize && new_y <= height as isize {
                if board[new_y as usize][new_x as usize].value == 9 {
                    return;
                }

                reveal_cell(board, new_x as usize, new_y as usize);
            }
        }
    }
}
