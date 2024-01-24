use crate::generation::generate_mines;
use crate::other::{Cell, CellState, GameState};
use crate::visuals::reveal_cell;
use crossterm::event::{read, Event, KeyCode};

pub fn controls(
    board: &mut Vec<Vec<Cell>>,
    pos_x: &mut usize,
    pos_y: &mut usize,
    game_state: &mut GameState,
    mines: usize,
    mines_generated: &mut bool,
) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

    if let Ok(Event::Key(key_event)) = read() {
        match key_event.code {
            KeyCode::Enter => {
                if !*mines_generated {
                    generate_mines(board, *pos_x, *pos_y, mines);
                    *mines_generated = true;
                } else if board[*pos_y][*pos_x].value == 9
                    && board[*pos_y][*pos_x].state != CellState::Flagged
                {
                    *game_state = GameState::Lost;
                }

                reveal_cell(board, *pos_x, *pos_y);
            }
            KeyCode::Char(' ') => {
                if board[*pos_y][*pos_x].state == CellState::Hidden {
                    board[*pos_y][*pos_x].state = CellState::Flagged;
                } else if board[*pos_y][*pos_x].state == CellState::Flagged {
                    board[*pos_y][*pos_x].state = CellState::Hidden;
                }

                reveal_cell(board, *pos_x, *pos_y);
            }
            KeyCode::Right if *pos_x < width => *pos_x += 1,
            KeyCode::Left if *pos_x > 0 => *pos_x -= 1,
            KeyCode::Up if *pos_y > 0 => *pos_y -= 1,
            KeyCode::Down if *pos_y < height => *pos_y += 1,
            _ => {}
        }
    }
}
