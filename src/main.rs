mod game;

fn main() {
    let mut tetris = game::init_game();
    tetris.run();
}
