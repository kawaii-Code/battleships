use std::fmt::Display;
use super::game_constants;

use game_constants as game_consts;

#[derive(Debug)]
pub enum CoordinateConversionError {
    CharacterLessThanBase(char),
    CharacterGreaterThanMax(char),
}

impl Display for CoordinateConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateConversionError::CharacterLessThanBase(base) => write!(f, "must be greater than {base}"),
            CoordinateConversionError::CharacterGreaterThanMax(max) => write!(f, "must be less than {max}"),
        }
    }
}

pub fn coordinate_to_usize(character: char) -> Result<usize, CoordinateConversionError> {
    let character = character as usize;
    let first = game_consts::FIRST_LETTER as usize;
    let last = game_consts::LAST_LETTER as usize;

    
    if character < first {
        Err(CoordinateConversionError::CharacterLessThanBase(game_consts::FIRST_LETTER))
    } else if character > last {
        Err(CoordinateConversionError::CharacterGreaterThanMax(game_consts::LAST_LETTER))
    } else {
        Ok(character - first)
    }
}
