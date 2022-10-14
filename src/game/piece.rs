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
                GridCell::BLOCK, GridCell::BLOCK, GridCell::BLOCK,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::O => &[
                GridCell::BLOCK, GridCell::BLOCK,
                GridCell::BLOCK, GridCell::BLOCK,
            ],
            PieceType::L => &[
                GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::BLOCK, GridCell::BLOCK, GridCell::EMPTY,
            ],
            PieceType::J => &[
                GridCell::EMPTY, GridCell::EMPTY, GridCell::BLOCK,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::BLOCK,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::BLOCK,
            ],
            PieceType::I => &[
                GridCell::EMPTY, GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::S => &[
                GridCell::EMPTY, GridCell::BLOCK, GridCell::BLOCK,
                GridCell::BLOCK, GridCell::BLOCK, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
            PieceType::Z => &[
                GridCell::BLOCK, GridCell::BLOCK, GridCell::EMPTY,
                GridCell::EMPTY, GridCell::BLOCK, GridCell::BLOCK,
                GridCell::EMPTY, GridCell::EMPTY, GridCell::EMPTY,
            ],
        }
    }
}

pub struct Pieces {
    shape : PieceType,
    pub rotation : i8,
    width: usize,
}

impl Pieces {
    pub fn make( t : PieceType ) -> Self {
        Self {
            rotation : rand::thread_rng().gen_range(0..=3),
            width:
                match &t {
                    PieceType::T | PieceType::L | PieceType::J | PieceType::S | PieceType::Z => 3,
                    PieceType::O => 2,
                    PieceType::I => 4,
                },
            shape : t,
        }
    }
    
    pub fn print_debug(&self) {
        println!("type {} -- ratation {}", 
                 match self.shape {
                     PieceType::T => "T",
                     PieceType::O => "O",
                     PieceType::L => "L",
                     PieceType::J => "J",
                     PieceType::I => "I",
                     PieceType::S => "S",
                     PieceType::Z => "Z",
                 }, self.rotation);
        for i in 0..self.width {
            for j in 0..self.width {
                print!("{}", match self.get(j,i) {
                    GridCell::EMPTY => 0,
                    _ => 1,
                });
            }
            println!("");
        }
    }

    pub fn get(&self, x:usize, y:usize) -> GridCell{
        self.shape.value()[
            match self.rotation {
                1 => self.width*(self.width -1) - self.width*x + y, // 90
                2 => (self.width*self.width-1) - self.width*y - x, // 180
                3 => self.width*x + self.width - y -1, // 270
                0 | _ => y * self.width + x
            }
        ]
    }
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
