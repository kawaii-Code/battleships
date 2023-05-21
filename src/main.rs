mod battleships;
mod utilities;
mod gameplay;

use battleships::{
    player::{Player, Victory},
    ship::{Ship, Rotation},
    shot::Shot,
    game_constants::FIELD_SIZE,
};

use utilities::input;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut player = Player::new(FIELD_SIZE);
    
    gameplay::place_ships(&mut player);

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
    
    let mut opponent_shot_y = 0;
    let mut opponent_shot_x = 0;
    loop {
        clear_screen();
        player.print();
        let shot = input::read_safe::<Shot>("Input a shot 'y:x'");

        match player.shoot(&mut opponent, shot.x, shot.y) {
            Ok(victory) => {
                match victory {
                    Victory::Win => {
                        clear_screen();
                        println!("You won!");
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

        match opponent.shoot(&mut player, opponent_shot_x, opponent_shot_y) {
            Ok(Victory::Win) => { 
                clear_screen();
                println!("You lost :(");
                break;
            },
            Ok(Victory::NotWin) => { }
            Err(message) => {
                panic!("Opponent shot in an invalid place: {message}");
            }
        }
        opponent_shot_y += 1;
        if opponent_shot_y >= FIELD_SIZE {
            opponent_shot_x += 1;
            opponent_shot_y = 0;
        }
    }

    println!("Final game state: ");
    println!("Your field: ");
    player.print();
    println!("Enemy field: ");
    opponent.print();
}
