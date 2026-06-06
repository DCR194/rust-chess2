use crate::board::Color;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new_piece(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = match (self.kind, self.color) {
            (PieceType::Pawn, Color::White) => "♙",
            (PieceType::Pawn, Color::Black) => "♟",
            (PieceType::Rook, Color::White) => "♖",
            (PieceType::Rook, Color::Black) => "♜",
            (PieceType::Knight, Color::White) => "♘",
            (PieceType::Knight, Color::Black) => "♞",
            (PieceType::Bishop, Color::White) => "♗",
            (PieceType::Bishop, Color::Black) => "♝",
            (PieceType::Queen, Color::White) => "♕",
            (PieceType::Queen, Color::Black) => "♛",
            (PieceType::King, Color::White) => "♔",
            (PieceType::King, Color::Black) => "♚",
        };
        write!(f, "{}", icon)
    }
}
