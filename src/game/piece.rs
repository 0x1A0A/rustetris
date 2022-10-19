use super::grid::GridCell;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

pub enum PieceType {
    T, O, L, J, I, S, Z
}

impl PieceType {
    pub fn value(&self) -> &[GridCell] {
        match *self {
            PieceType::T => &[
                GridCell::MOVING, GridCell::MOVING, GridCell::MOVING,
                GridCell::EMPTY, GridCell::MOVING, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::O => &[
                GridCell::MOVING, GridCell::MOVING,
                GridCell::MOVING, GridCell::MOVING,
            ],
            PieceType::L => &[
                GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::MOVING, GridCell::MOVING, GridCell::EMPTY,
            ],
            PieceType::J => &[
                GridCell::EMPTY, GridCell::EMPTY, GridCell::MOVING,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::MOVING,
                GridCell::EMPTY, GridCell::MOVING, GridCell::MOVING,
            ],
            PieceType::I => &[
                GridCell::EMPTY, GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::MOVING, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::S => &[
                GridCell::EMPTY, GridCell::MOVING, GridCell::MOVING,
                GridCell::MOVING, GridCell::MOVING, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::Z => &[
                GridCell::MOVING, GridCell::MOVING, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::MOVING, GridCell::MOVING,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
        }
    }
}

pub struct Pieces {
    shape : PieceType,
    pub rotation : i8,
    pub width: isize,
    pub pos: (isize, isize),
}

impl Pieces {
    pub fn make( t : PieceType ) -> Self {
        let w = match &t {
            PieceType::T | PieceType::L | PieceType::J | PieceType::S | PieceType::Z => 3,
            PieceType::O => 2,
            PieceType::I => 4,
        };
        let x = rand::thread_rng()
            .gen_range(2..(super::grid::BOARD_WIDTH - w -1) );
        Self {
            rotation : rand::thread_rng().gen_range(0..=3),
            width: w,
            shape : t,
            pos : (x, -w),
        }
    }

    pub fn get(&self, x:isize, y:isize) -> GridCell{
        self.shape.value()[
            match self.rotation {
                1 => self.width*(self.width -1) - self.width*x + y, // 90
                2 => (self.width*self.width-1) - self.width*y - x, // 180
                3 => self.width*x + self.width - y -1, // 270
                0 | _ => y * self.width + x
            } as usize
        ]
    }

    pub fn rand() -> Self { Self::make( rand::random() ) }

    pub fn rotate(&mut self, dir:i8) {
        self.rotation += 4 + dir;
        self.rotation %= 4;
    }

    pub fn slide(&mut self, dir:isize) { self.pos.0 += dir; }

    pub fn drop(&mut self) { self.pos.1 += 1; }

    pub fn raise(&mut self) { self.pos.1 += -1; }
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..7) {
            0 => PieceType::T,
            1 => PieceType::O,
            2 => PieceType::L,
            3 => PieceType::J,
            4 => PieceType::I,
            5 => PieceType::S,
            _ => PieceType::Z,
        }
    }
}
