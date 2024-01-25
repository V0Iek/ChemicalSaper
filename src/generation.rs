use crate::other::{Cell, CellState};
use rand::Rng;

fn set_mine(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let width = board[0].len();
    let height = board.len();

    board[y][x].value = 9;

    for k in -1..=1 {
        for l in -1..=1 {
            let new_y = y as isize + k;
            let new_x = x as isize + l;

            if new_y >= 0 && new_y < height as isize && new_x >= 0 && new_x < width as isize {
                if board[new_y as usize][new_x as usize].value != 9 {
                    board[new_y as usize][new_x as usize].value += 1;
                }
            }
        }
    }
}

pub fn generate_mines(board: &mut Vec<Vec<Cell>>, mut mines: usize) {
    let width = board[0].len() - 1;
    let height = board.len() - 1;

    while mines > 0 {
        let x = rand::thread_rng().gen_range(0..=width);
        let y = rand::thread_rng().gen_range(0..=height);
        if board[y][x].value != 9 {
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
