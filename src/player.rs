use crate::field::{PlayerField, CheckField, FieldCell, CheckedCell};
use crate::Ship;

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

    pub fn place_ship(&mut self, ship: &Ship) {
        self.player_field.place_ship(ship);
    }

    pub fn can_place(&self, ship: &Ship) -> bool {
        self.player_field.can_place(ship)
    }

    pub fn shoot(&mut self, opponent: &Player, x: usize, y: usize) {
        if true {
            self.checked_field.mark(x, y, CheckedCell::Hit);
        } else {
            self.checked_field.mark(x, y, CheckedCell::Miss);
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
