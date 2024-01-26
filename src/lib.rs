pub mod enums;
pub use enums::{CellState, GameState};

pub mod structs;
pub use structs::{Cell, Minesweeper};

pub mod visuals;
pub use visuals::{clear_terminal, reveal_cell, show_board};

pub mod generation;
pub use generation::{generate_board, generate_mines};

pub mod controls_mod;
pub use controls_mod::controls;
