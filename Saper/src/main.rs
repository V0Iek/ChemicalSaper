use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::Stylize;
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

static mut BOARD: [[Cell; 16]; 16] = [[Cell {
    value: 0,
    state: CellState::Hidden,
}; 16]; 16];

static mut POS_X: usize = 0;
static mut POS_Y: usize = 0;

static mut END: GameState = GameState::InProgress;

fn generate_board() -> bool {
    for x in 0..=15 {
        for y in 0..=15 {
            unsafe {
                BOARD[y][x].value = 0;
                BOARD[y][x].state = CellState::Hidden;
            }
        }
    }
    true
}

fn set_mine(x: usize, y: usize) -> bool {
    unsafe {
        BOARD[y][x].value = 9;

        for k in 0..=2 {
            for l in 0..=2 {
                if y + k > 0 && x + l > 0 && y + k < 17 && x + l < 17 {
                    if BOARD[y + k - 1][x + l - 1].value == 9 {
                        continue;
                    }

                    BOARD[y + k - 1][x + l - 1].value += 1;
                }
            }
        }
    }

    true
}

fn generate_mines() {
    let mut pos_x;
    let mut pos_y;
    let mut count: i32 = 40;

    while count > 0 {
        pos_x = rand::thread_rng().gen_range(0..=15);
        pos_y = rand::thread_rng().gen_range(0..=15);
        unsafe {
            if BOARD[pos_y][pos_x].value != 9 {
                set_mine(pos_x, pos_y);
                count -= 1;
            }
        }
    }
}

fn clear_terminal() {
    // Wykonaj polecenie do wyczyszczenia ekranu
    execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");

    // Ustaw kursor w lewym górnym rogu
    print!("\x1B[H");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn show_board() {
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    clear_terminal();

    for i in 0..=15 {
        for j in 0..=15 {
            unsafe {
                if POS_Y == i && POS_X == j {
                    if BOARD[i][j].state == CellState::Revealed {
                        if BOARD[i][j].value == 0 {
                            print!("{} ", "O".green());
                        } else {
                            print!("{} ", BOARD[i][j].value.to_string().green());
                        }
                    } else if BOARD[i][j].state == CellState::Flagged {
                        print!("{} ", "P".green());
                    } else {
                        print!("{} ", "#".green());
                    }
                } else {
                    if BOARD[i][j].state == CellState::Revealed {
                        if BOARD[i][j].value == 0 {
                            print!("  ");
                        } else {
                            print!("{} ", BOARD[i][j].value);
                        }
                    } else if BOARD[i][j].state == CellState::Flagged {
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

fn reveal_cell(mut x: usize, mut y: usize) {
    unsafe {
        if x > 15 || y > 15 || BOARD[y][x].state != CellState::Hidden || BOARD[y][x].value == 9 {
            return;
        }

        if BOARD[y][x].state == CellState::Hidden {
            BOARD[y][x].state = CellState::Revealed;
        }
    }

    for i in 0..=2 {
        for j in 0..=2 {
            if x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                reveal_cell(x + i, y + j);
            }
        }
    }
}

fn controls() {
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    if let Ok(Event::Key(key_event)) = read() {
        unsafe {
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Enter {
                if BOARD[POS_Y][POS_X].value == 9 {
                    END = GameState::Loosed;
                }

                reveal_cell(POS_X, POS_Y);
                show_board();
            }
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Char(' ') {
                if BOARD[POS_Y][POS_X].state == CellState::Hidden {
                    BOARD[POS_Y][POS_X].state = CellState::Flagged;
                } else if BOARD[POS_Y][POS_X].state == CellState::Flagged {
                    BOARD[POS_Y][POS_X].state = CellState::Hidden;
                }

                reveal_cell(POS_X, POS_Y);
                show_board();
            }

            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Right {
                if POS_X < 15 {
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
                if POS_Y < 15 {
                    POS_Y += 1;
                }
            }
        }
    }
}

fn check_if_win() -> bool {
    for i in 0..=15 {
        for j in 0..=15 {
            unsafe {
                if BOARD[i][j].value != 9 && BOARD[i][j].state != CellState::Revealed {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    generate_board();
    generate_mines();

    unsafe {
        while END == GameState::InProgress {
            controls();
            if check_if_win() {
                END = GameState::Won;
            }
        }

        if END == GameState::Won {
            println!("You won!");
        }
        if END == GameState::Loosed {
            reveal_cell(POS_X, POS_Y);
            println!("Boom! You loose");
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
