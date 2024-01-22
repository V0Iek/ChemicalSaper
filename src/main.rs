use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use rand::Rng;
use std::io::{self, Write};
use std::usize;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy)]
struct Cell {
    value: i32,
    state: CellState,
}

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Loosed,
    Won,
}

static mut POS_X: usize = 0;
static mut POS_Y: usize = 0;

static mut END: GameState = GameState::InProgress;

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

fn generate_mines(board: &mut Vec<Vec<Cell>>, mut mines: usize) {
    let width = board[0].len();
    let height = board.len();

    let mut pos_x;
    let mut pos_y;

    while mines > 0 {
        pos_x = rand::thread_rng().gen_range(0..=width - 1);
        pos_y = rand::thread_rng().gen_range(0..=height - 1);
        if board[pos_y][pos_x].value != 9 {
            set_mine(board, pos_x, pos_y);
            mines -= 1;
        }
    }
}

fn generate_board(width: usize, height: usize, mines: usize) -> Vec<Vec<Cell>> {
    let mut board: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                value: 0,
                state: CellState::Hidden
            };
            width
        ];
        height
    ];

    generate_mines(&mut board, mines);

    board
}

fn clear_terminal() {
    // Wykonaj polecenie do wyczyszczenia ekranu
    execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");

    // Ustaw kursor w lewym górnym rogu
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

fn show_board(board: &mut Vec<Vec<Cell>>) {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    clear_terminal();

    let width = board[0].len() - 1;
    let height = board.len() - 1;

    for i in 0..=height {
        for j in 0..=width {
            unsafe {
                if POS_Y == i && POS_X == j {
                    if board[i][j].state == CellState::Revealed {
                        if board[i][j].value == 0 {
                            print!("{} ", "O".green());
                        } else {
                            print!("{} ", board[i][j].value.to_string().green());
                        }
                    } else if board[i][j].state == CellState::Flagged {
                        print!("{} ", "P".green());
                    } else {
                        print!("{} ", "#".green());
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
        }
        print!("\n");
    }
    unsafe {
        println!("Cursor position:");
        println!("X: {}", POS_X);
        println!("Y: {}", POS_Y);
    }

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
}

fn reveal_cell(board: &mut Vec<Vec<Cell>>, mut x: usize, mut y: usize) {
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

fn controls(board: &mut Vec<Vec<Cell>>) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    if let Ok(Event::Key(key_event)) = read() {
        unsafe {
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Enter {
                if board[POS_Y][POS_X].value == 9 && board[POS_Y][POS_X].state != CellState::Flagged
                {
                    END = GameState::Loosed;
                }

                reveal_cell(board, POS_X, POS_Y);
                show_board(board);
            }
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Char(' ') {
                if board[POS_Y][POS_X].state == CellState::Hidden {
                    board[POS_Y][POS_X].state = CellState::Flagged;
                } else if board[POS_Y][POS_X].state == CellState::Flagged {
                    board[POS_Y][POS_X].state = CellState::Hidden;
                }

                reveal_cell(board, POS_X, POS_Y);
                show_board(board);
            }

            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Right {
                if POS_X < width {
                    POS_X += 1;
                }
            }
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Left {
                if POS_X > 0 {
                    POS_X -= 1;
                }
            }
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Up {
                if POS_Y > 0 {
                    POS_Y -= 1;
                }
            }
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Down {
                if POS_Y < height {
                    POS_Y += 1;
                }
            }
        }
    }
}

fn check_if_win(board: &mut Vec<Vec<Cell>>) -> bool {
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

fn main() {
    let mut choose = String::new();
    let mut board: Vec<Vec<Cell>>;

    clear_terminal();

    println!("Choose the difficulty level:");
    println!("1. 8x8 board, 10 mines");
    println!("2. 16x16 board, 40 mines");
    println!("3. 30x16 board, 99 mines");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");

    let difficulty = choose.trim().parse::<usize>();

    match difficulty {
        Ok(1) => board = generate_board(8, 8, 10),
        Ok(2) => board = generate_board(16, 16, 40),
        Ok(3) => board = generate_board(30, 16, 99),
        _ => {
            println!("Invalid choice. Exiting.");
            return;
        }
    }

    show_board(&mut board);

    unsafe {
        while END == GameState::InProgress {
            controls(&mut board);
            if check_if_win(&mut board) {
                END = GameState::Won;
            }
        }

        if END == GameState::Won {
            println!("You won!");
        }
        if END == GameState::Loosed {
            reveal_cell(&mut board, POS_X, POS_Y);
            println!("Boom! You loose");
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
