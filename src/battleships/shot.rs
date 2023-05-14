use std::{fmt::Display, str::FromStr, num::ParseIntError, char::ParseCharError};

use crate::utilities::conversions::{self, CoordinateConversionError};

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
            Ok(value) => value,
            Err(error) => return Err(ParseShotError::ConversionError(error)),
        };

        let x = match data.next() {
            Some(value) => value,
            None => return Err(ParseShotError::MissingInfo),
        };
        let x = match x.parse::<usize>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShotError::ParseIntError(error)),
        };

        Ok(Shot { x, y })
    }
}

pub enum ParseShotError {
    MissingInfo,
    ParseIntError(ParseIntError),
    ParseCharError(ParseCharError),
    ConversionError(CoordinateConversionError),
}

impl Display for ParseShotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseShotError::MissingInfo => write!(f, "missing some info about the shot. Maybe you forgot a ':'?"),
            ParseShotError::ParseIntError(inner) => write!(f, "can't understand the x coordinate: {inner}"),
            ParseShotError::ParseCharError(inner) => write!(f, "can't understand the y coordinate: {inner}"),
            ParseShotError::ConversionError(inner) => write!(f, "the y coordinate is invalid: {inner}"),
        }
    }
}
