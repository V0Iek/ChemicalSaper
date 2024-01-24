use crate::other::{Cell, CellState};
use rand::Rng;

fn set_mine(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let width = board[0].len() + 1;
    let height = board.len() + 1;

    board[y][x].value = 9;

    for k in 0..=2 {
        for l in 0..=2 {
            if y + k > 0
                && x + l > 0
                && y + k < height
                && x + l < width
                && board[y + k - 1][x + l - 1].value != 9
            {
                board[y + k - 1][x + l - 1].value += 1;
            }
        }
    }
}

pub fn generate_mines(board: &mut Vec<Vec<Cell>>, pos_x: usize, pos_y: usize, mut mines: usize) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    let mut x;
    let mut y;

    while mines > 0 {
        x = rand::thread_rng().gen_range(0..=width);
        y = rand::thread_rng().gen_range(0..=height);
        if board[y][x].value != 9 || !(y == pos_y && x == pos_x) {
            set_mine(board, x, y);
            mines -= 1;
        }
    }
}

pub fn generate_board(width: usize, height: usize) -> Vec<Vec<Cell>> {
    vec![
        vec![
            Cell {
                value: 0,
                state: CellState::Hidden
            };
            width
        ];
        height
    ]
}
