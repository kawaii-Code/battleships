use std::{fmt::Display, str::FromStr, num::ParseIntError, char::ParseCharError};

use super::game_constants::{FIRST_LETTER, LAST_LETTER};
use crate::utilities::conversions;

pub struct Shot { 
    pub x: usize,
    pub y: usize 
}

impl FromStr for Shot {
    type Err = ParseShotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split(':');

        let y = match data.next() {
            Some(value) => value,
            None => return Err(ParseShotError::MissingInfo),
        };
        let y = match y.parse::<char>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShotError::ParseCharError(error)),
        };
        let y = match conversions::coordinate_to_usize(y) {
            Some(value) => value,
            None => return Err(ParseShotError::ConversionError),
        };

        let x = match data.next() {
            Some(value) => value,
            None => return Err(ParseShotError::MissingInfo),
        };
        let x = match x.parse::<usize>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShotError::ParseIntError(error)),
        };
        let x = x - 1;

        Ok(Shot { x, y })
    }
}

pub enum ParseShotError {
    MissingInfo,
    ParseIntError(ParseIntError),
    ParseCharError(ParseCharError),
    ConversionError,
}

impl Display for ParseShotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseShotError::MissingInfo => write!(f, "missing some info about the shot. Maybe you forgot a ':'?"),
            ParseShotError::ParseIntError(inner) => write!(f, "can't understand the x coordinate: {inner}"),
            ParseShotError::ParseCharError(inner) => write!(f, "can't understand the y coordinate: {inner}"),
            ParseShotError::ConversionError => write!(f, "the y coordinate is invalid: coordinate must be in range: {FIRST_LETTER}..{LAST_LETTER}"),
        }
    }
}
