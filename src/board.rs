use crate::moves::PieceMove;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{pieces::Piece, player::PlayerState};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, EnumIter, Debug)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

#[derive(Clone, Copy, EnumIter, Debug)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

pub struct Square {
    pub file: File,
    pub rank: Rank,
}

pub struct Board([[Option<Piece>; 8]; 8]);

impl Board {
    pub fn new_default() -> Self {
        use crate::pieces::PieceType::*;
        Self([
            [
                Some(Piece::new_piece(Rook, Color::Black)),
                Some(Piece::new_piece(Knight, Color::Black)),
                Some(Piece::new_piece(Bishop, Color::Black)),
                Some(Piece::new_piece(Queen, Color::Black)),
                Some(Piece::new_piece(King, Color::Black)),
                Some(Piece::new_piece(Bishop, Color::Black)),
                Some(Piece::new_piece(Knight, Color::Black)),
                Some(Piece::new_piece(Rook, Color::Black)),
            ],
            [
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
                Some(Piece::new_piece(Pawn, Color::Black)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
                Some(Piece::new_piece(Pawn, Color::White)),
            ],
            [
                Some(Piece::new_piece(Rook, Color::White)),
                Some(Piece::new_piece(Knight, Color::White)),
                Some(Piece::new_piece(Bishop, Color::White)),
                Some(Piece::new_piece(Queen, Color::White)),
                Some(Piece::new_piece(King, Color::White)),
                Some(Piece::new_piece(Bishop, Color::White)),
                Some(Piece::new_piece(Knight, Color::White)),
                Some(Piece::new_piece(Rook, Color::White)),
            ],
        ])
    }

    pub fn get_piece_as_ref(&self, rank: Rank, file: File) -> Option<&Piece> {
        self.0[rank as usize][file as usize].as_ref()
    }

    pub fn make_move(&mut self, piece_move: PieceMove) {
        if let Some(piece) =
            self.0[piece_move.from.rank as usize][piece_move.from.file as usize].take()
        {
            self.0[piece_move.to.rank as usize][piece_move.to.file as usize] = Some(piece);
        }
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = 3;
        writeln!(f, "")?;
        for rank in Rank::iter().rev() {
            write!(f, "{:<indent$}", format!("{:?} ", rank as u8 + 1))?;
            for file in File::iter() {
                if let Some(piece) = self.board.get_piece_as_ref(rank, file) {
                    piece.fmt(f)?;
                } else {
                    write!(f, ".")?;
                }
                write!(f, " ")?;
            }
            writeln!(f, "")?;
        }
        write!(f, "{:<indent$}", " ")?;
        for file in File::iter() {
            write!(f, "{:?} ", file)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Current turn: {:?}", self.current_turn)?;
        writeln!(f, "White state: {:?}", self.white_state)?;
        writeln!(f, "Black state: {:?}", self.black_state)?;
        Ok(())
    }
}

pub struct GameState {
    board: Board,
    // TODO: implement this as a piece -> positions map
    // white_pieces_pos: SmallVec<(Rank, File)>,
    // black_pieces_pos: SmallVec<(Rank, File)>,
    current_turn: Color,
    white_state: PlayerState,
    black_state: PlayerState,
}

impl GameState {
    pub fn new_default() -> Self {
        Self {
            board: Board::new_default(),
            // white_pieces_pos: SmallVec::new(),
            // black_pieces_pos: SmallVec::new(),
            current_turn: Color::White,
            white_state: PlayerState::new(Color::White),
            black_state: PlayerState::new(Color::Black),
        }
    }

    pub fn make_move(&mut self, piece_move: PieceMove) {
        self.board.make_move(piece_move);
        self.current_turn = self.current_turn.opposite();
    }
}
