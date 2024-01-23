pub mod other;
pub use other::{Cell, CellState, GameState};

pub mod functions;
pub use functions::{
    check_if_win, clear_terminal, generate_board, generate_mines, reveal_cell, show_board,
};

pub mod controls_mod;
pub use controls_mod::controls;
