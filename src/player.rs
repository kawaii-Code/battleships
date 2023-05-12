use crate::field::{PlayerField, CheckField, FieldCell, CheckedCell};
use crate::Ship;

pub enum Victory {
    Win,
    NotWin,
}

pub struct Player {
    player_field: PlayerField,
    checked_field: CheckField,
    size: usize,
}

impl Player {
    pub fn new(player_field_size: usize) -> Player {
        let player_field = PlayerField::new(player_field_size);
        let checked_field = CheckField::new(player_field_size);

        Player {
            player_field,
            checked_field,
            size: player_field_size,
        }
    }

    pub fn lost(&self) -> bool {
        for x in 0..self.size {
            for y in 0..self.size {
                if self.player_field.is_ship_on(x as isize, y as isize) {
                    return false;
                }
            }
        }
        true
    }

    pub fn place_ship(&mut self, ship: &Ship) {
        self.player_field.place_ship(ship);
    }

    pub fn can_place(&self, ship: &Ship) -> bool {
        self.player_field.can_place(ship)
    }

    pub fn take_damage(&mut self, x: usize, y: usize) -> Result<CheckedCell, String> {
        let shot_cell = self.player_field.at(x, y);
        match shot_cell {
            FieldCell::Ship => {
                if !self.player_field.will_ship_survive_after_shot(x, y) {
                    self.player_field.mark_dead(x, y);
                    return Ok(CheckedCell::Kill);
                }            

                self.player_field.mark_hit(x, y);
                Ok(CheckedCell::Hit)
            }
            FieldCell::EnemyMiss => Err("Already shot in this place".to_string()),
            FieldCell::Hit => Err("Already shot in this place!".to_string()),
            FieldCell::Dead => Err("Already shot in this place!".to_string()),
            FieldCell::Empty => { 
                self.player_field.mark_enemy_miss(x, y);
                Ok(CheckedCell::Miss)
            }
        }
    }

    pub fn shoot(&mut self, opponent: &mut Player, x: usize, y: usize) -> Result<Victory, String> {
        match opponent.take_damage(x, y) {
            Ok(hit_result) => { 
                self.checked_field.mark(x, y, hit_result);
                if opponent.lost() {
                    Ok(Victory::Win)
                } else {
                    Ok(Victory::NotWin)
                }
            },
            Err(message) => Err(message),
        }
    }

    pub fn print(&self) {
        let chars: Vec<char> = ('A'..'Z').take(self.size).collect();
        let field_separator = "\t";

        let number_header: String = (1..=self.size)
                .map(|x| format!(" {x} "))
                .collect();
        let line = format!(
            "   {number_header}{field_separator}   {number_header} "
        );
        println!("{line}");

        for y in 0..self.size {
            let marker = chars[y];

            let player_field_line: String = (0..self.size)
                    .map(|x| self.player_field.at(x, y))
                    .map(|cell| match cell {
                        FieldCell::Empty => "[ ]",
                        FieldCell::Ship => "[#]",
                        FieldCell::EnemyMiss => "[*]",
                        FieldCell::Hit => "[!]",
                        FieldCell::Dead => "[X]",
                    })
                    .collect();

            let checked_field_line: String = (0..self.size)
                    .map(|x| self.checked_field.at(x, y))
                    .map(|cell| match cell {
                        CheckedCell::Unchecked => "[ ]",
                        CheckedCell::Miss => "[*]",
                        CheckedCell::Hit => "[!]",
                        CheckedCell::Kill => "[X]",
                    })
                    .collect();
            
            let line = format!(
                " {marker} {player_field_line}{field_separator} {marker} {checked_field_line} "
            );
            println!("{line}");
        }
    }
}
