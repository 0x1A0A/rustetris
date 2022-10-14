mod game;

fn main() {
    let tetris = game::init_game();
    tetris.board.print_debug();
    let r : game::piece::PieceType = rand::random();
    let p = game::piece::Pieces::make(r);
    p.print_debug();
}
