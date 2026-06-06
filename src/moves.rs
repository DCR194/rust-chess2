use crate::board::Square;
use crate::pieces::Piece;

pub enum SpecialMove {
    Castling,
    Promotion(Piece),
    None,
}

pub struct PieceMove {
    pub from: Square,
    pub to: Square,
    pub special: SpecialMove,
}
