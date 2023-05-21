use std::{str::FromStr, num::ParseIntError, fmt::Display, char::ParseCharError};
use crate::utilities::conversions;
use super::game_constants::{FIRST_LETTER, LAST_LETTER};

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Rotation {
    Horizontal,
    Vertical
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h" => Ok(Rotation::Horizontal),
            "v" => Ok(Rotation::Vertical),
            _ => Err(ParseRotationError::InvalidInput),
        }
    }
}

pub struct Ship {
    pub length: usize,
    pub x: usize,
    pub y: usize,
    pub rotation: Rotation,
}

impl FromStr for Ship {
    type Err = ParseShipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ship_iter = s.split(':');

        let length = match ship_iter.next() {
            Some(value) => value,
            None => return Err(ParseShipError::MissingInfo),
        };
        let length = match length.parse::<usize>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShipError::ParseIntError(error)),
        };

        let y = match ship_iter.next() {
            Some(value) => value,
            None => return Err(ParseShipError::MissingInfo),
        };
        let y = match y.parse::<char>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShipError::ParseCharError(error)),
        };
        let y = match conversions::coordinate_to_usize(y) {
            Some(value) => value,
            None => return Err(ParseShipError::ConversionToCharError),
        };

        let x = match ship_iter.next() {
            Some(value) => value,
            None => return Err(ParseShipError::MissingInfo),
        };
        let x = match x.parse::<usize>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShipError::ParseIntError(error)),
        };
        let x = x - 1;

        let rotation = match ship_iter.next() {
            Some(value) => value,
            None => return Err(ParseShipError::MissingInfo),
        };
        let rotation = match rotation.parse::<Rotation>() {
            Ok(value) => value,
            Err(error) => return Err(ParseShipError::ParseRotationError(error)),
        };

        Ok(Ship {
            length, x, y, rotation
        })
    }
}

pub enum ParseShipError {
    MissingInfo,
    ParseIntError(ParseIntError),
    ParseCharError(ParseCharError),
    ConversionToCharError,
    ParseRotationError(ParseRotationError),
}

impl Display for ParseShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseShipError::MissingInfo => write!(f, "Some information about the ship is missing!"),
            ParseShipError::ParseIntError(inner) => write!(f, "Can't understand the x coordinate: {inner}"),
            ParseShipError::ParseCharError(inner) => write!(f, "Can't understand the y coordinate: {inner}"),
            ParseShipError::ConversionToCharError => write!(f, "The y coordinate was wrong: coordinate must be in range {FIRST_LETTER}..{LAST_LETTER}"),
            ParseShipError::ParseRotationError(inner) => write!(f, "The rotation inputted is wrong: {inner}"),
        }
    }
}

pub enum ParseRotationError {
    InvalidInput,
}

impl Display for ParseRotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRotationError::InvalidInput => write!(f, "rotation must be one of 'v'/'h'"),
        }
    }
}
