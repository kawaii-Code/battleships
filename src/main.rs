mod battleships;

use std::{io, collections::HashMap, str::FromStr};
use battleships::{
    player::{Player, Victory},
    ship::{Ship, Rotation},
};

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
        Some(coordinate) => {
            let ord = coordinate as usize;
            if ord < 'A' as usize {
                return Err("The y coordinate must be a letter!".to_string());
            } else if ord > LAST_LETTER as usize {
                return Err("The y coordinate was greater than possible: max is {LAST_LETTER}".to_string());
            }

            Ok(ord - 'A' as usize)
        }
        None => Err("No y coordinate provided!".to_string()),
    }
}

fn input_ship() -> Result<Ship, String> {
    let ship = input("Input a ship 'length:x:y': ");
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
const LAST_LETTER: char = ('A' as usize + FIELD_SIZE) as u8 as char;

fn main() {
    let mut player = Player::new(FIELD_SIZE);

    let mut ships_to_place = HashMap::new();
    ships_to_place.insert(4usize, 1);
    ships_to_place.insert(3, 2);
    ships_to_place.insert(2, 3);
    ships_to_place.insert(1, 4);

    println!("Please input {SHIP_COUNT} ships in format 'length:x:y', e.g. '4:3:B': ");

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

    println!("Your ship placement!");
    player.print();

    let mut opponent = Player::new(FIELD_SIZE);

    let opponent_ships = vec![
        Ship { length: 4, x: 0,  y: 0,  rotation: Rotation::Vertical },
        Ship { length: 3, x: 4,  y: 1,  rotation: Rotation::Horizontal },
        Ship { length: 3, x: 9, y: 4,  rotation: Rotation::Vertical },
        Ship { length: 2, x: 0,  y: 9,  rotation: Rotation::Horizontal },
        Ship { length: 2, x: 4,  y: 3,  rotation: Rotation::Vertical },
        Ship { length: 2, x: 7,  y: 9,  rotation: Rotation::Horizontal },
        Ship { length: 1, x: 1,  y: 6,  rotation: Rotation::Horizontal },
        Ship { length: 1, x: 3,  y: 6,  rotation: Rotation::Horizontal },
        Ship { length: 1, x: 9, y:  0, rotation: Rotation::Horizontal },
        Ship { length: 1, x: 6,  y: 5,  rotation: Rotation::Horizontal },
    ];

    for ship in opponent_ships {
        opponent.place_ship(&ship);
    }

    loop {
        let shot = input("Input a shot in format 'x:y'");
        let mut shot = shot.split(':');
        let x = shot.next().unwrap().parse::<usize>().unwrap() - 1;
        let y = to_index(shot.next().unwrap()).unwrap();

        match player.shoot(&mut opponent, x, y) {
            Ok(victory) => {
                match victory {
                    Victory::Win => {
                        println!("You won!!!");
                        break;
                    }
                    Victory::NotWin => {
                    }
                }
            }
            Err(message) => {
                eprintln!("{message}");
                continue;
            }
        }

        player.print();
    }

    println!("Final game state: ");
    player.print();
}
