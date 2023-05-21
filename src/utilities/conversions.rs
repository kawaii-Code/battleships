use crate::battleships::game_constants::{FIRST_LETTER, LAST_LETTER};

pub fn coordinate_to_usize(character: char) -> Option<usize> {
    let character = character as usize;
    let first = FIRST_LETTER as usize;
    let last = LAST_LETTER as usize;

    if character < first || character > last {
        None
    } else {
        Some(character - first)
    }
}

pub fn usize_to_coordinate(coordinate: usize) -> char {
    (FIRST_LETTER as u8 + coordinate as u8) as char
}
