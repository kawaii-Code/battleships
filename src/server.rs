mod battleships;
mod utilities;
mod gameplay;

use std::net::{TcpListener, TcpStream};

use battleships::{
    player::Player,
    game_constants::FIELD_SIZE, 
};

use utilities::unsafe_net::{self, MAGIC_BYTE_PLACE, MAGIC_BYTE_SHOOT, MAGIC_BYTE_GET_SHOT};

const ADDRESS: &str = "127.0.0.1:6969";

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let mut player = Player::new(FIELD_SIZE);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        start_game(stream, &mut player);
    }
}

fn start_game(mut opponent_stream: TcpStream, player: &mut Player) {
    loop {
        let mut opponent_message_buf = [0u8; 3];
        unsafe_net::read_blocking(&mut opponent_stream, &mut opponent_message_buf);

        println!("Read the stream: {}", opponent_message_buf[0]);
        if opponent_message_buf[0] == MAGIC_BYTE_PLACE {
            gameplay::place_ships(player);
        } else if opponent_message_buf[0] == MAGIC_BYTE_SHOOT {
            shoot_loop(&mut opponent_stream, player);
        } 
    }
}

fn shoot_loop(stream: &mut TcpStream, player: &mut Player) {
    loop {
        shoot_client(stream, player);
        take_shot(stream, player);
    }
}

fn shoot_client(stream: &mut TcpStream, player: &mut Player) {
    let shot = gameplay::read_shot(player);
    unsafe_net::send(stream, &[MAGIC_BYTE_GET_SHOT, shot.x as u8, shot.y as u8]);
}

fn take_shot(stream: &mut TcpStream, player: &mut Player) {
    let mut shot_buf = [0u8; 3];
    unsafe_net::read_blocking(stream, &mut shot_buf);

    if shot_buf[0] == MAGIC_BYTE_GET_SHOT {
        // TODO: Also send the result of this, with errors, etc.
        player.take_damage(shot_buf[1].into(), shot_buf[2].into());
        player.print();
    }
}
