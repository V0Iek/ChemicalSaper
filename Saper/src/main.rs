use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use rand::Rng;
use std::io::{self, Write};
use std::usize;

#[derive(Clone, Copy)]
struct Cell {
    value: i32,
    reveled: bool,
}

static mut BOARD: [[Cell; 16]; 16] = [[Cell {
    value: 0,
    reveled: false,
}; 16]; 16];

static mut POS_X: usize = 0;
static mut POS_Y: usize = 0;

static mut END: i32 = 0;

fn generate_board() -> bool {
    for x in 0..=15 {
        for y in 0..=15 {
            unsafe {
                BOARD[y][x].value = 0;
                BOARD[y][x].reveled = false;
            }
        }
    }
    true
}

fn set_mine(x: usize, y: usize) -> bool {
    unsafe {
        BOARD[y][x].value = 9;
    }

    for k in 0..=2 {
        for l in 0..=2 {
            if x + k < 1 || x + k > 16 {
                continue;
            }
            if y + l < 1 || y + l > 16 {
                continue;
            }

            unsafe {
                if BOARD[x + k - 1][y + l - 1].value == 9 {
                    continue;
                }

                BOARD[x + k - 1][y + l - 1].value += 1;
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
                set_mine(pos_y, pos_x);
                count -= 1;
            }
        }
    }
}

fn clear_terminal() {
    // Wyczyszczenie ekranu przy użyciu sekwencji ANSI
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn show_board() {
    clear_terminal();

    unsafe {
        for i in 0..=15 {
            for j in 0..=15 {
                /*if i == POS_X && j == POS_Y {
                    print!("# ");
                } else {*/
                if BOARD[i][j].reveled {
                    if BOARD[i][j].value == 0 {
                        print!("  ");
                    } else {
                        print!("{} ", BOARD[i][j].value);
                    }
                } else {
                    print!("# ");
                }
                //}
            }
            print!("\n");
        }

        println!("Cursor position:");
        println!("X: {}", POS_X);
        println!("Y: {}", POS_Y);
    }
}

fn reveal_cell(mut x: usize, mut y: usize) {
    unsafe {
        if
        /*x < 0 ||*/
        x > 15 {
            return;
        }
        if
        /*y < 0 ||*/
        y > 15 {
            return;
        }
        if BOARD[y][x].reveled {
            return;
        }

        if BOARD[y][x].reveled == false {
            BOARD[y][x].reveled = true;
        }
    }

    for i in 0..=2 {
        for j in 0..=2 {
            if x > 0 && y > 0 {
                x -= 1;
                y -= 1;
                reveal_cell(y + i, x + j);
            }
        }
    }
}

fn controls() {
    if let Ok(Event::Key(key_event)) = read() {
        unsafe {
            if key_event.modifiers == KeyModifiers::NONE && key_event.code == KeyCode::Enter {
                if BOARD[POS_Y][POS_X].value == 9 {
                    END = 2;
                }

                reveal_cell(POS_Y, POS_X);
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
    let mut mines: i32 = 0;

    for i in 0..=15 {
        for j in 0..=15 {
            unsafe {
                if BOARD[i][j].reveled == false {
                    mines += 1;
                }
            }
        }
    }

    if mines == 40 {
        true
    } else {
        false
    }
}

fn main() {
    generate_board();
    generate_mines();

    unsafe {
        while END == 0 {
            controls();
            if check_if_win() {
                END = 1;
            }
        }

        if END == 1 {
            println!("You won!");
        }
        if END == 2 {
            println!("Boom! You loose");
        }
    }
}
