pub const BOARD_WIDTH : usize = 12;
pub const BOARD_HEIGHT : usize = 21; 

#[derive(Copy, Clone)]
pub enum GridCell {
    EMPTY,
    WALL,
    BLOCK,
}

pub struct Grid {
    data : [GridCell; BOARD_WIDTH*BOARD_HEIGHT ],
}

pub fn build_grid() -> Grid {
    let mut g = Grid {
        data: [ GridCell::EMPTY; BOARD_WIDTH*BOARD_HEIGHT ], 
    };

    for i in 0..21 {
        for j in 0..12 {
            if i == 20 || j == 0 || j == 11 {
                g.data[i*BOARD_WIDTH + j] = GridCell::WALL;
            }
        }
    }

    return g;
}

impl Grid {
    pub fn get(&self, x:usize, y:usize) -> &GridCell {
        return &self.data[ y*BOARD_WIDTH + x];
    }
    pub fn print_debug(&self) {
        for i in 0..21 {
            for j in 0..12 {
                print!("{}", match self.get(j,i) {
                    GridCell::EMPTY => 0,
                    _ => 1,
                });
            } println!("");
        }
    }
}

