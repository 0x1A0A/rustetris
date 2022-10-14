pub mod grid;
pub mod piece;

pub struct Game {
    pub board : grid::Grid,
}

pub fn init_game() -> Game {
    return Game {
        board : grid::build_grid(), 
    };
}

