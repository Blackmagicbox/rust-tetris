use self::piece::{Piece, Kind as PieceKind};

mod piece;

pub struct Engine {
    board: Board,
    bag: Vec<Piece>,
}

impl Engine {
    pub(crate) fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: vec![],
        }
    }
}

struct Board([bool;Self::SIZE]);
impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;


    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
}