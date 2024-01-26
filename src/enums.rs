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
