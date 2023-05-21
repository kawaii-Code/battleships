use std::collections::HashMap;

use crate::{
    battleships::{player::Player, game_constants::SHIP_COUNT, ship::Ship, field::ShipPlacementError, shot::Shot}, 
    utilities::{input, pretty_output}
};

pub fn read_shot(player: &mut Player) -> Shot {
    pretty_output::clear_screen();
    player.print();
    input::read_safe::<Shot>("Input a shot 'y:x'")
}

pub fn place_ships(player: &mut Player) {
    let mut ships_left = HashMap::new();
    ships_left.insert(4usize, 1);
    ships_left.insert(3, 2);
    ships_left.insert(2, 3);
    ships_left.insert(1, 4);

    for _ in 0..SHIP_COUNT {
        pretty_output::clear_screen();
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

