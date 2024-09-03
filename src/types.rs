use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Hash, Eq, Ord, FromPrimitive, ToPrimitive)]
pub enum Piece {
    PAWN = 0,
    KNIGHT = 1,
    BISHOP = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Hash, Eq, Ord, FromPrimitive, ToPrimitive)]
pub enum Color {
    WHITE = 0,
    BLACK = 1
}

pub struct Move {
    src: u8,
    dst: u8,
    promote: bool,
    promotion_piece: Piece
}

impl Move {
    pub fn new(src:u8, dst:u8) -> Move{
        Move {
            src: src,
            dst: dst,
            promote: false,
            promotion_piece: Piece::PAWN
        }
    }
    pub fn promotion(src:u8, dst:u8, promotion_piece: Piece) -> Move {
        Move {
            src: src,
            dst: dst,
            promote: true,
            promotion_piece: promotion_piece
        }
    }
}