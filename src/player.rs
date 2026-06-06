use crate::board::*;

#[derive(Debug)]
pub enum CastleRights {
    // Long,
    // Short,
    Both,
    // None,
}

#[derive(Debug)]
pub struct PlayerState {
    // TODO: add move_number, king_in_check, material_points, time
    pub castle_rights: CastleRights,
    pub color: Color,
}

impl PlayerState {
    pub fn new(color: Color) -> Self {
        Self {
            castle_rights: CastleRights::Both,
            color,
        }
    }
}
