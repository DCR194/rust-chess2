use crate::board::Square;
use crate::pieces::Piece;

#[derive(Debug, Clone, PartialEq)]
pub enum AlgebraicAction {
    Move,
    Capture,
    Promote,
    ShortCastle,
    LongCastle,
}

pub struct AlgebraicRepresentation {
    pub action: AlgebraicAction,
    pub source_piece: Option<Piece>,
    pub source_square: Option<Square>,
    pub target_piece: Option<Piece>,
    pub target_square: Option<Square>,
}
