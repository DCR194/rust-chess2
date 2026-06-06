mod board;
mod moves;
mod pieces;
mod player;

fn main() {
    let mut game_state = board::GameState::new_default();
    dbg!(&game_state);
    let piece_move = moves::PieceMove {
        from: board::Square {
            file: board::File::E,
            rank: board::Rank::Two,
        },
        to: board::Square {
            file: board::File::E,
            rank: board::Rank::Four,
        },
        special: moves::SpecialMove::None,
    };
    game_state.make_move(piece_move);
    dbg!(&game_state);
    let piece_move = moves::PieceMove {
        from: board::Square {
            file: board::File::E,
            rank: board::Rank::Seven,
        },
        to: board::Square {
            file: board::File::E,
            rank: board::Rank::Five,
        },
        special: moves::SpecialMove::None,
    };
    game_state.make_move(piece_move);
    dbg!(&game_state);
}
