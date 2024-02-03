use lib::{game_loop, init_game, next_round, reveal_cell, show_board, Cell, CellState, GameState};

fn main() {
    let mut game_state = GameState::InProgress;
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;
    let mut board: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                value: 0,
                state: CellState::Hidden
            };
            0
        ];
        0
    ];
    let mut mines: usize = 0;
    let mut mines_generated = false;

    while game_state != GameState::Ended {
        init_game(
            &mut game_state,
            &mut board,
            &mut mines_generated,
            &mut mines,
        );

        show_board(&board, pos_x, pos_y, mines);

        while game_state == GameState::InProgress {
            game_loop(
                &mut game_state,
                &mut board,
                &mut pos_x,
                &mut pos_y,
                &mut mines_generated,
                mines,
            );
        }

        if game_state == GameState::Lost {
            reveal_cell(&mut board, pos_x, pos_y);
            show_board(&board, pos_x, pos_y, mines);

            crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
            println!("Boom! You lost");

            game_state = next_round();
        }
        if game_state == GameState::Won {
            crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
            println!("You won!");

            game_state = next_round();
        }
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
