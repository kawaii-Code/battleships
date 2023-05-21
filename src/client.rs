mod battleships;
mod utilities;
mod gameplay;

use std::net::TcpStream;

use battleships::{
    player::Player,
    game_constants::FIELD_SIZE,
};

use utilities::{unsafe_net::{self, MAGIC_BYTE_PLACE, MAGIC_BYTE_SHOOT, MAGIC_BYTE_GET_SHOT}, pretty_output::clear_screen};

const ADDRESS: &str = "127.0.0.1:6969";

fn main() {
    let mut player = Player::new(FIELD_SIZE);
    let mut opponent_stream = TcpStream::connect(ADDRESS).unwrap();
    unsafe_net::send(&mut opponent_stream, &[MAGIC_BYTE_PLACE]);

    gameplay::place_ships(&mut player);

    unsafe_net::send(&mut opponent_stream, &[MAGIC_BYTE_SHOOT]);

    shoot_loop(&mut opponent_stream, &mut player);
}

fn shoot_loop(stream: &mut TcpStream, player: &mut Player) {
    clear_screen();
    loop {
        take_shot(stream, player);
        clear_screen();
        player.print();

        shoot_server(stream, player);
    }
}

fn shoot_server(stream: &mut TcpStream, player: &mut Player) {
    let shot = gameplay::read_shot(player);
    unsafe_net::send(stream, &[MAGIC_BYTE_GET_SHOT, shot.x as u8, shot.y as u8]);
}

fn take_shot(stream: &mut TcpStream, player: &mut Player) {
    let mut shot_buf = [0u8; 3];
    unsafe_net::read_blocking(stream, &mut shot_buf);
    
    if shot_buf[0] == MAGIC_BYTE_GET_SHOT {
        player.take_damage(shot_buf[1].into(), shot_buf[2].into());
        player.print();
    }
}
