use crate::other::{Cell, CellState};
use rand::Rng;

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
