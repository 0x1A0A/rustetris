use raylib::prelude::*;
use rand::Rng;

pub mod grid;
pub mod piece;

pub struct Game {
    pub board : grid::Grid,
    pub play_piece : Option<piece::Pieces>,
}

pub fn init_game() -> Game {
    return Game {
        board : grid::build_grid(), 
        play_piece : None,
    };
}

impl Game {
    pub fn run(&mut self) {
        let (mut rl, thread) = raylib::init()
            .size(640,480)
            .title("dfalse - TETRIS")
            .build();

        while !rl.window_should_close() {
            match &self.play_piece {
                None => {
                    self.play_piece = Some(piece::Pieces::rand());
                    self.play_piece
                        .as_mut()
                        .unwrap()
                        .pos.0 = rand::thread_rng()
                        .gen_range(2..(grid::BOARD_WIDTH - self.play_piece.as_ref().unwrap().width -1) );
                },
                Some(_) => {
                },
            } 
            
            self.handle_slide(&mut rl);
            self.handle_rotate(&mut rl);
            self.set_board();

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::RAYWHITE);
            d.draw_text("TETRIS", 10,10, 10, Color::GRAY);
            self.draw_board(&mut d);
        }
    }

    fn set_board(&mut self) {
        let p: &piece::Pieces = self.play_piece.as_ref().unwrap();
        for i in 0..grid::BOARD_HEIGHT-1 {
            for j in 1..grid::BOARD_WIDTH-1 {
                if j >= p.pos.0 && j < p.pos.0 + p.width && i >= p.pos.1 && i < p.pos.1 + p.width {
                   self.board.set(j, i, p.get(j-p.pos.0, i-p.pos.1)); 
                } else {
                    match self.board.get(j,i) {
                        grid::GridCell::MOVING => {
                            self.board.set(j,i,grid::GridCell::EMPTY);
                        },
                        _ => {},
                    }
                }
            }
        }
    }
    
    fn handle_slide(&mut self, rl:&mut RaylibHandle) {
        let p: &mut piece::Pieces = self.play_piece.as_mut().unwrap();
        let mv =
            rl.is_key_pressed(KeyboardKey::KEY_L) as isize - 
            rl.is_key_pressed(KeyboardKey::KEY_H) as isize;

        p.pos.0 += mv;
    }

    fn handle_rotate(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_A) {
            self.play_piece.as_mut().unwrap().rotate(1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            self.play_piece.as_mut().unwrap().rotate(-1);
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
                   _ => {
                       d.draw_rectangle(
                           start.0 + j as i32 * size,
                           start.1 + i as i32 * size,
                           size, size,
                           Color::GRAY
                           );
                   },
                }
            }
        }
    }
}
