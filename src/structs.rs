use crate::enums::CellState;
use crate::generation::{generate_board, generate_mines};
// use glib::signal::Inhibit;
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
        let mut board = generate_board(width, height);

        generate_mines(&mut board, mine_count);

        Minesweeper {
            width,
            height,
            mine_count,
            board,
        }
    }

    fn handle_cell_click(&self, x: usize, y: usize) {
        println!("Cell {} {} clicked", x, y);
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

                label.connect_button_press_event(move |_, _| {
                    self.handle_cell_click(col, row);
                    Inhibit(false)
                });

                grid.attach(&label, col as i32, row as i32, 1, 1);
            }
        }

        grid.show_all();
    }
}
