use crate::enums::CellState;
use gtk::prelude::*;
use gtk::{Grid, Label};

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: i32,
    pub state: CellState,
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    mine_count: usize,
    board: Vec<Vec<Cell>>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        let board = vec![
            vec![
                Cell {
                    value: 0,
                    state: CellState::Hidden
                };
                width
            ];
            height
        ];

        Minesweeper {
            width,
            height,
            mine_count,
            board,
        }
    }

    pub fn display_board(&self, grid: &Grid) {
        grid.foreach(|child| grid.remove(child));

        for row in 0..self.height {
            for col in 0..self.width {
                let label = Label::new(None);

                match self.board[row][col].state {
                    CellState::Hidden => label.set_text(" "),
                    CellState::Flagged => label.set_text("P"),
                    CellState::Revealed => label.set_text(&self.board[row][col].value.to_string()),
                }

                grid.attach(&label, col as i32, row as i32, 1, 1);
            }
        }

        grid.show_all();
    }
}
