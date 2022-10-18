pub const BOARD_WIDTH : isize = 12;
pub const BOARD_HEIGHT : isize = 21; 

#[derive(Copy, Clone)]
pub enum GridCell {
    EMPTY,
    WALL,
    BLOCK,
    MOVING,
}

pub struct Grid {
    data : [GridCell; (BOARD_WIDTH*BOARD_HEIGHT) as usize],
}

pub fn build_grid() -> Grid {
    let mut g = Grid {
        data: [ GridCell::EMPTY; (BOARD_WIDTH*BOARD_HEIGHT) as usize ], 
    };

    for i in 0..21 {
        for j in 0..12 {
            if i == 20 || j == 0 || j == 11 {
                g.data[(i*BOARD_WIDTH + j) as usize] = GridCell::WALL;
            }
        }
    }

    return g;
}

impl Grid {
    pub fn get(&self, x:isize, y:isize) -> &GridCell {
        return &self.data[ (y*BOARD_WIDTH + x) as usize];
    }

    pub fn set(&mut self, x:isize, y:isize, v:GridCell) {
        self.data[ (y*BOARD_WIDTH + x) as usize] = v;
    }

    pub fn is_row_full(&self, row:isize) -> bool {
        for j in 1..BOARD_WIDTH-1 {
            if let GridCell::EMPTY = self.get(j, row) {
                return false;
            }
        }
        return true;
    }
}

