use gtk::prelude::*;
use gtk::{Grid, Window, WindowType};
use lib::Minesweeper;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rusty Minesweeper");
    window.set_default_size(512, 512);

    let grid = Grid::new();
    grid.set_row_spacing(2);
    grid.set_column_spacing(2);

    let minesweeper = Minesweeper::new(16, 16, 40);
    minesweeper.display_board(&grid);

    window.add(&grid);

    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    window.show_all();

    gtk::main();
}
