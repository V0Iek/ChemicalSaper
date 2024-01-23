#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(PartialEq)]
pub enum GameState {
    InProgress,
    Lost,
    Won,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub value: i32,
    pub state: CellState,
}
