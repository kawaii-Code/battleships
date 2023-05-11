use std::{io, collections::HashMap};

mod field;
mod player;

use player::{Player};

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Rotation {
    Horizontal,
    Vertical
}

pub struct Ship {
    length: usize,
    x: usize,
    y: usize,
    rotation: Rotation,
}

fn input(message: &str) -> String {
    println!("{message}");

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => buf.trim().to_owned(),
        Err(e) => { 
            println!("{e}");
            String::new()
        }
    }
}

fn input_rotation(message: &str) -> Result<Rotation, String> {
    let input = input(message);

    match input.as_str().trim() {
        "h" => Ok(Rotation::Horizontal),
        "v" => Ok(Rotation::Vertical),
        _ => Err("Please input only the character 'h' or 'v'!".to_string()),
    }
}

fn to_index(letter: &str) -> Result<usize, String> {
    match letter.chars().next() {
        Some(coordinate) => Ok(coordinate as usize - 'A' as usize),
        None => Err("No y coordinate provided!".to_string()),
    }
}

fn input_ship() -> Result<Ship, String> {
    let ship = input("Input a ship in format 'length:x:y': ");
    let mut ship_iter = ship.split(':');

    let length = match ship_iter.next() {
        Some(value) => value,
        None => return Err("Nothing was inputted!".to_string()),
    };
    let length = match length.parse::<usize>() {
        Ok(value) => value,
        Err(error) => return Err(error.to_string()),
    };

    let x = match ship_iter.next() {
        Some(value) => value,
        None => return Err("Expected a colon after length!".to_string()),
    };
    let x = match x.parse::<usize>() {
        Ok(value) => value - 1,
        Err(error) => return Err(error.to_string()),
    };

    let y = match ship_iter.next() {
        Some(value) => value,
        None => return Err("Expected a colon after x!".to_string()),
    };
    let y = match to_index(y) {
        Ok(value) => value,
        Err(message) => return Err(message),
    };

    let rotation = match input_rotation("Input a rotation (v/h): ") {
        Ok(rotation) => rotation,
        Err(message) => return Err(message),
    };

    Ok(Ship { 
        length, x, y, rotation
    })
}

const FIELD_SIZE: usize = 10;
const SHIP_COUNT: u32 = 10;

fn main() {
    let mut player = Player::new(FIELD_SIZE);

    let mut ships_to_place = HashMap::new();
    ships_to_place.insert(4usize, 1);
    ships_to_place.insert(3, 2);
    ships_to_place.insert(2, 3);
    ships_to_place.insert(1, 4);

    for _ in 0..SHIP_COUNT {
        loop {
            player.print();

            let ship = match input_ship() {
                Ok(ship) => ship,
                Err(message) => {
                    eprintln!("{message}");
                    continue;
                }
            };

            if !player.can_place(&ship) {
                eprintln!("Invalid ship placement! Try again.");
                continue;
            }

            match ships_to_place.get_mut(&ship.length) {
                Some(ships_left) => {
                    if *ships_left == 0 {
                        eprintln!("Can't place any more ships of length {}!", ship.length);
                        continue;
                    } else {
                        *ships_left -= 1;
                    }
                }
                None => {
                    eprintln!("Length {} is not available!", ship.length);
                    continue;
                }
            }

            player.place_ship(&ship);
            break;
        }
    }

    let opponent = Player::new(FIELD_SIZE);

    player.shoot(&opponent, 0, 0);
    player.shoot(&opponent, 1, 1);
    player.shoot(&opponent, 1, 4);
    player.shoot(&opponent, 8, 4);
    player.shoot(&opponent, 9, 9);

    player.print();
}
