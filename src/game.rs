use raylib::prelude::*;

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

impl Game {
    pub fn run(&self) {
        let (mut rl, thread) = raylib::init()
            .size(640,480)
            .title("dfalse - TETRIS")
            .build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_text("TETRIS", 10,10, 10, Color::GRAY);
            self.draw_board(&mut d);
        }
    }

    fn draw_board(&self, d:&mut RaylibDrawHandle) {
        const size:i32= 20;
        const start:(i32,i32) = (10,30);

        // verticle line
        for i in 2..grid::BOARD_WIDTH-1 {
            d.draw_line( 
                size*i as i32 +start.0,
                start.1,
                size*i as i32 +start.0,
                grid::BOARD_HEIGHT as i32 *size +start.1,
                Color::GRAY
                );
        }

        // horizontal line
        for i in 1..grid::BOARD_HEIGHT-1 {
            d.draw_line( 
                start.0,
                size*i as i32 +start.1,
                grid::BOARD_WIDTH as i32 * size +start.0,
                size*i as i32 +start.1,
                Color::GRAY
                );
        }

        //draw block
        for i in 0..grid::BOARD_HEIGHT {
            for j in 0..grid::BOARD_WIDTH {
                match self.board.get(j,i) {
                   grid::GridCell::EMPTY => {
                   },
                   grid::GridCell::WALL => {
                       d.draw_rectangle(
                           start.0 + j as i32 * size,
                           start.1 + i as i32 * size,
                           size, size,
                           Color::BLACK
                           );
                   },
                   grid::GridCell::BLOCK => {
                   },
                }
            }
        }
    }
}
