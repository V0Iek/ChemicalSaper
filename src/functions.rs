use crate::other::{Cell, CellState};
use crossterm::execute;
use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use rand::Rng;
use std::io::{self, Write};

fn set_mine(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let width = board[0].len();
    let height = board.len();

    board[y][x].value = 9;

    for k in 0..=2 {
        for l in 0..=2 {
            if y + k > 0 && x + l > 0 && y + k < height + 1 && x + l < width + 1 {
                if board[y + k - 1][x + l - 1].value == 9 {
                    continue;
                }

                board[y + k - 1][x + l - 1].value += 1;
            }
        }
    }

    true
}

pub fn generate_mines(board: &mut Vec<Vec<Cell>>, pos_x: usize, pos_y: usize, mut mines: i32) {
    let width = board[0].len();
    let height = board.len();

    let mut x;
    let mut y;

    while mines > 0 {
        x = rand::thread_rng().gen_range(0..=width - 1);
        y = rand::thread_rng().gen_range(0..=height - 1);
        if board[y][x].value != 9 || (y != pos_y && x != pos_x) {
            set_mine(board, x, y);
            mines -= 1;
        }
    }
}

pub fn generate_board(width: usize, height: usize) -> Vec<Vec<Cell>> {
    let board: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                value: 0,
                state: CellState::Hidden
            };
            width
        ];
        height
    ];

    board
}

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

pub fn check_if_win(board: &mut Vec<Vec<Cell>>) -> bool {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    for i in 0..=height {
        for j in 0..=width {
            if board[i][j].value != 9 && board[i][j].state != CellState::Revealed {
                return false;
            }
        }
    }
    true
}
