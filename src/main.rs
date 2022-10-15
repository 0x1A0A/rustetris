mod game;

fn main() {
    let tetris = game::init_game();
    tetris.run();
}
