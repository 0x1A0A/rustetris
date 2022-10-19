use raylib::prelude::*;

pub mod grid;
pub mod piece;

pub struct Game {
    pub board : grid::Grid,
    pub play_piece : Option<piece::Pieces>,
    drop_time: f64,
    down_hold: f64,
}

pub fn init_game() -> Game {
    return Game {
        board : grid::build_grid(), 
        play_piece : None,
        drop_time: 0.0,
        down_hold: 0.0,
    };
}

impl Game {
    pub fn run(&mut self) {
        let (mut rl, thread) = raylib::init()
            .size(640,480)
            .title("dfalse - TETRIS")
            .build();

        rl.set_target_fps(60);

        while !rl.window_should_close() {
            match &self.play_piece {
                None => {
                    self.play_piece = Some(piece::Pieces::rand());
                    // spawn piece
                    while self.is_out_of_bound() {
                        self.play_piece.as_mut().unwrap().drop();
                        if self.check_collision() {
                            break;
                        }
                    }

                    if self.is_out_of_bound() { 
                        println!("DEAD");
                        self.board.reset();
                        self.play_piece = None;
                        continue;
                    }
                },
                Some(_) => {
                },
            } 

            self.handle_slide(&mut rl);
            self.handle_rotate(&mut rl);
            self.set_board(); 

            if rl.is_key_down(KeyboardKey::KEY_K) {
                self.down_hold += rl.get_frame_time() as f64;
            }

            if self.drop_time >= 0.5 || 
                rl.is_key_pressed(KeyboardKey::KEY_K) || 
                    self.down_hold >= 0.05 {
                        self.move_down();
                        self.drop_time = 0.0;
                        self.down_hold = 0.0;
                    } else {
                        self.drop_time += rl.get_frame_time() as f64;
                    }

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::RAYWHITE);
            d.draw_text("TETRIS", 10,10, 10, Color::GRAY);
            self.draw_board(&mut d);
            d.draw_text(& format!("{}",self.drop_time), 70,10, 10, Color::RED);
        }
    }

    fn move_down(&mut self) {
        self.play_piece.as_mut().unwrap().drop();

        if self.check_collision() {
            let p: &mut piece::Pieces = self.play_piece.as_mut().unwrap();
            // can't move down so it is the end of this round 
            p.raise();
            // change all moving to block
            for i in 0..p.width {
                for j in 0..p.width {
                    if let grid::GridCell::MOVING = p.get(j,i) {
                        self.board.set(p.pos.0+j, p.pos.1+i, grid::GridCell::BLOCK);
                    }
                }
            }

            self.play_piece = None;
            let mut complete = 0;
            // completion checking
            let mut i: isize = grid::BOARD_HEIGHT-2;
            while i >= 0 {
                if self.board.is_row_full(i) {
                    complete+=1;
                    for k in (0..=i).rev() {
                        for j in 1..grid::BOARD_WIDTH-1 {
                            if k-1< 0 {
                                self.board.set(j,k, grid::GridCell::EMPTY);
                            } else {
                                let p = self.board.get( j,k - 1);
                                self.board.set(j,k, *p);
                            }
                        }
                    }
                    i+=1;
                }
                i-=1;
            }


            if complete != 0 {
                println!("::{complete} Line Deleted");
            }
        }
    }

    fn set_board(&mut self) {
        let p: &piece::Pieces = self.play_piece.as_ref().unwrap();
        for i in 0..grid::BOARD_HEIGHT-1 {
            for j in 1..grid::BOARD_WIDTH-1 {
                // add moving block to the board
                if j >= p.pos.0 && j < p.pos.0 + p.width && i >= p.pos.1 && i < p.pos.1 + p.width {
                    if let grid::GridCell::MOVING = p.get(j-p.pos.0, i-p.pos.1) {
                        self.board.set(j, i, p.get(j-p.pos.0, i-p.pos.1)); 
                    } else {
                        match self.board.get(j,i) {
                            grid::GridCell::MOVING => {
                                self.board.set(j,i,grid::GridCell::EMPTY);
                            },
                            _ => {},
                        }
                    }
                } else {
                    // remove old moving block from the board
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
        let mv =
            rl.is_key_pressed(KeyboardKey::KEY_L) as isize - 
            rl.is_key_pressed(KeyboardKey::KEY_J) as isize;

        if mv == 0 { return; }

        // check for left or right collision
        self.play_piece.as_mut().unwrap().slide(mv);

        if self.check_collision() {
            self.play_piece.as_mut().unwrap().slide(-mv);
        }
    }

    fn check_collision(& self) -> bool {
        let p: & piece::Pieces = self.play_piece.as_ref().unwrap();
        for i in 0..p.width {
            for j in 0..p.width {
                if i + p.pos.1 < 0 { continue; }
                if let grid::GridCell::MOVING = p.get(j,i) {
                    if let grid::GridCell::WALL | grid::GridCell::BLOCK =
                        self.board.get(j+p.pos.0,i+p.pos.1) {
                            return true;
                        }
                }
            }
        }
        return false;
    }

    fn is_out_of_bound(&self) -> bool {
        let p: & piece::Pieces = self.play_piece.as_ref().unwrap();
        for i in 0..p.width {
            for j in 0..p.width {
                if let grid::GridCell::MOVING = p.get(j,i) {
                    if p.pos.1 + i < 0 {
                        return true;  
                    }
                }
            }
        }
        return false;
    }

    fn handle_rotate(&mut self, rl: &mut RaylibHandle) {
        let mut a = 0;
        if rl.is_key_pressed(KeyboardKey::KEY_A) { a = 1; }
        if rl.is_key_pressed(KeyboardKey::KEY_D) { a = -1; }
        self.play_piece.as_mut().unwrap().rotate(a);
        if self.check_collision() {
            self.play_piece.as_mut().unwrap().rotate(-a);
        }
    }

    fn draw_board(&self, d:&mut RaylibDrawHandle) {
        const SIZE:i32= 20;
        const START:(i32,i32) = (10,30);

        // verticle line
        for i in 2..grid::BOARD_WIDTH-1 {
            d.draw_line( 
                SIZE*i as i32 +START.0,
                START.1,
                SIZE*i as i32 +START.0,
                grid::BOARD_HEIGHT as i32 *SIZE +START.1,
                Color::GRAY
                );
        }

        // horizontal line
        for i in 1..grid::BOARD_HEIGHT-1 {
            d.draw_line( 
                START.0,
                SIZE*i as i32 +START.1,
                grid::BOARD_WIDTH as i32 * SIZE +START.0,
                SIZE*i as i32 +START.1,
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
                            START.0 + j as i32 * SIZE,
                            START.1 + i as i32 * SIZE,
                            SIZE, SIZE,
                            Color::GRAY
                            );
                    },
                }
            }
        }
    }
}
