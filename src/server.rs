use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::collections::HashMap;

use battleships::{
    player::{Player, Victory},
    ship::{Ship, Rotation},
    shot::Shot,
    field::ShipPlacementError,
};

use utilities::{
    input,
    game_constants::{SHIP_COUNT, FIELD_SIZE},
};

mod battleships;
mod utilities;

const ADDRESS: &str = "127.0.0.1:6969";

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    // TODO: Burn this code
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let mut player = Player::new(FIELD_SIZE);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut player);
    }
}

fn handle_connection(mut stream: TcpStream, player: &mut Player) {
    loop {
        let mut buf = [0u8; 3];
        let count = stream.read(&mut buf).unwrap();
        if count == 0 {
            continue;
        }
        
        if buf[0] != 0 {
            println!("Read the stream: {}", buf[0]);
            if buf[0] == 1 {
                place_ships(player);
            } else if buf[0] == 2 {
                shoot(&mut stream, player);
            } else if buf[0] == 3 {
                println!("Getting shot at {}, {}", buf[1], buf[2]);
            }
        }
    }
}

fn shoot(stream: &mut TcpStream, player: &mut Player) {
    loop {
        clear_screen();
        player.print();
        let shot = input::read_safe::<Shot>("Input a shot 'y:x'");
        stream.write(&[3u8, shot.x as u8, shot.y as u8]);

        let mut shot_buf = [0u8; 3];
        loop {
            let count = stream.read(&mut shot_buf).unwrap();
            if count == 0 {
                continue;
            }
    
            if shot_buf[0] == 3 {
                println!("Getting shot at {}, {}!", shot_buf[1], shot_buf[2]);
                break;
            }
        }
        
        // TODO: Also send the result of this, with errors, etc.
        player.take_damage(shot_buf[1].into(), shot_buf[2].into());
    }
}

fn place_ships(player: &mut Player) {
    let mut ships_left = HashMap::new();
    ships_left.insert(4usize, 1);
    ships_left.insert(3, 2);
    ships_left.insert(2, 3);
    ships_left.insert(1, 4);

    for _ in 0..SHIP_COUNT {
        clear_screen();
        player.print();

        let ship: Ship = input::read_while("Input a ship 'length:y:x:rotation': ", |ship| {
            if let Err(error) = player.can_place(ship) {
                return Err(error);
            }

            if ships_left.get(&ship.length).unwrap_or(&0) == &0 {
                return Err(ShipPlacementError::NoShipsOfLengthLeft(ship.length));
            }

            Ok(())
        });

        *ships_left.get_mut(&ship.length).expect("This shouldn't happen") -= 1;

        player.place_ship(&ship);
    }

    println!("Your ship placement:");
    player.print();
}
