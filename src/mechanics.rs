use crate::controls_mod::controls;
use crate::other::{Cell, CellState, GameState};
use crate::visuals::show_board;

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

pub fn game_loop(
    game_state: &mut GameState,
    board: &mut Vec<Vec<Cell>>,
    pos_x: &mut usize,
    pos_y: &mut usize,
    mines_generated: &mut bool,
    mines: usize,
) {
    controls(
        &mut *board,
        &mut *pos_x,
        &mut *pos_y,
        &mut *game_state,
        &mut *mines_generated,
        mines,
    );

    if check_if_win(&mut *board) {
        *game_state = GameState::Won;
    }

    show_board(&board, *pos_x, *pos_y, mines);
}
