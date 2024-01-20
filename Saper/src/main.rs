use rand::Rng;

struct Cell {
    value: i32,
    reveled: bool,
}

fn GenerateBoard(board: [[Cell; 16]; 16]) -> bool {
    for x in 0..=15 {
        for y in 0..=15 {
            board[x][y].value = 0;
            board[x][y].reveled = false;
        }
    }
    true
}

fn SetMine(board: [[Cell; 16]; 16], x: i32, y: i32) -> bool {
    board[x][y].value = 9;

    for k in -1..=1 {
        for l in -1..=1 {
            if x + k < 0 || y + l < 0 {
                continue;
            }
            if x + k > 15 || y + l > 15 {
                continue;
            }

            if board[x + k][y + l] == 9 {
                continue;
            }

            board[x + k][y + l].value += 1;
        }
    }

    true
}

fn GenerateMines(board: [[Cell; 16]; 16]) {
    let mut pos_x: i32;
    let mut pos_y: i32;
    let mut count: i32 = 40;

    while count > 0 {
        pos_x = rand::thread_rng().gen_range(0..=15);
        pos_y = rand::thread_rng().gen_range(0..=15);

        if board[pos_x][pos_y].value != 9 {
            SetMine(pos_x, pos_y);
            count -= 1;
        }
    }
}

fn main() {
    let mut board: [[Cell; 16]; 16];
}
