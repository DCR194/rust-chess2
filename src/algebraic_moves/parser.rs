use crate::{
    algebraic_moves::algebraic_representation::AlgebraicRepresentation,
    board::{File, Rank},
};

#[derive(Debug)]
pub struct AlgebraicMoveParser {
    input: String,
    position: usize,
}

type ParserSavedState = usize;

impl AlgebraicMoveParser {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    pub fn parse(&mut self) -> Result<AlgebraicRepresentation, ()> {
        Err(())
    }

    fn save_state(&self) -> ParserSavedState {
        self.position
    }

    fn restore_state(&mut self, saved_state: ParserSavedState) {
        self.position = saved_state;
    }

    pub fn try_parse<T, F>(&mut self, f: F) -> Result<T, ()>
    where
        F: FnOnce(&mut Self) -> Result<T, ()>,
    {
        let saved_state = self.save_state();
        let res = f(self);
        if res.is_err() {
            self.restore_state(saved_state);
        }
        res
    }

    fn expect_castles(&mut self) -> Result<AlgebraicToken, ()> {
        // Always parse short castles `O-O` first
        if self.expect_str("O-O", true).is_ok() {
            // Then, if we know we already have the O-O check if we have the
            // remaining `-O` that would make it a long castle `O-O-O`
            if self.expect_str("-O", true).is_ok() {
                return Ok(AlgebraicToken::CastlesLong);
            }
            return Ok(AlgebraicToken::CastlesShort);
        }
        Err(())
    }

    fn expect_char(&mut self, c: char) -> Result<char, ()> {
        if self.next_char()? == c {
            Ok(c)
        } else {
            Err(())
        }
    }

    fn expect_str<T>(&mut self, s: &str, expected: T) -> Result<T, ()> {
        if s.chars().all(|c| self.expect_char(c).is_ok()) {
            Ok(expected)
        } else {
            Err(())
        }
    }

    pub fn next_char(&mut self) -> Result<char, ()> {
        self.input
            .as_bytes()
            .get(self.position)
            .map(|c| {
                self.position += 1;
                *c as char
            })
            .ok_or(())
    }

    // NOTE(descalante): Not quite sure if this is really necessary.
    pub fn peek_char(&mut self) -> Result<char, ()> {
        self.input
            .as_bytes()
            .get(self.position)
            .map(|c| *c as char)
            .ok_or(())
    }

    pub fn next_token(&mut self) -> Result<AlgebraicToken, ()> {
        let c = self.peek_char()?;
        let res = match c {
            'K' => Ok(AlgebraicToken::King),
            'Q' => Ok(AlgebraicToken::Queen),
            'B' => Ok(AlgebraicToken::Bishop),
            'N' => Ok(AlgebraicToken::Knight),
            '#' => Ok(AlgebraicToken::Check),
            'x' => Ok(AlgebraicToken::Takes),
            '=' => Ok(AlgebraicToken::Promote),
            'a'..='h' => Ok(AlgebraicToken::File(Self::char_to_file(c))),
            '1'..='8' => Ok(AlgebraicToken::Rank(Self::char_to_rank(c))),
            _ => Err(()),
        };
        if res.is_ok() {
            self.position += 1;
            res
        } else {
            self.expect_castles()
        }
    }

    fn char_to_rank(c: char) -> Rank {
        debug_assert!(c.is_ascii_digit());
        let i = c.to_digit(10).unwrap() as u8;
        debug_assert!(i >= 1 && i <= 8);
        Rank::try_from(i - 1).unwrap()
    }

    fn char_to_file(c: char) -> File {
        debug_assert!(c.is_ascii_lowercase());
        let i = c as u8 - b'a';
        debug_assert!(i <= 7);
        File::try_from(i).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgebraicToken {
    King,
    Queen,
    Bishop,
    Knight,
    Rank(Rank),
    File(File),
    CastlesShort,
    CastlesLong,
    Takes,
    Check,
    Promote,
}
