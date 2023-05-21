use std::fmt::Display;

use super::ship::{Ship, Rotation};
use super::game_constants::{FIELD_SIZE, FIRST_LETTER, LAST_LETTER};

use crate::utilities::{
    conversions
};

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum FieldCell {
    Empty,
    Ship,
    EnemyMiss,
    Hit,
    Dead,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum CheckedCell {
    Unchecked,
    Miss,
    Hit,
    Kill,
}

pub struct CheckField {
    field: Vec<Vec<CheckedCell>>,
    size: usize,
}

impl CheckField {
    pub fn new(size: usize) -> CheckField {
        let field = vec![vec![CheckedCell::Unchecked; size]; size];

        CheckField {
            field,
            size
        }
    }

    pub fn mark(&mut self, x: usize, y: usize, marker: CheckedCell) {
        if marker == CheckedCell::Kill {
            self.mark_kill(x, y);
        } else {
            self.field[x][y] = marker;
        }
    }

    pub fn at(&self, x: usize, y: usize) -> CheckedCell {
        assert!(x < self.size && y < self.size);
        self.field[x][y]
    }

    fn mark_kill(&mut self, x: usize, y: usize) {
        self.field[x][y] = CheckedCell::Hit;
        mark_kill_recursive(self, x as isize, y as isize);

        fn mark_kill_recursive(me: &mut CheckField, x: isize, y: isize) {
            if x < 0 || y < 0 || x >= me.size as isize || y >= me.size as isize {
                return;
            }

            let cell = me.at(x as usize, y as usize);

            if cell == CheckedCell::Hit{
                me.field[x as usize][y as usize] = CheckedCell::Kill;
                mark_kill_recursive(me, x - 1, y);
                mark_kill_recursive(me, x + 1, y);
                mark_kill_recursive(me, x, y + 1);
                mark_kill_recursive(me, x, y - 1);
            }
        }
    }
}

pub struct PlayerField {
    field: Vec<Vec<FieldCell>>,
    size: usize,
}

impl PlayerField {
    pub fn new(size: usize) -> PlayerField {
        let field = vec![vec![FieldCell::Empty; size]; size];

        PlayerField {
            field,
            size,
        }
    }

    #[inline]
    pub fn at(&self, x: usize, y: usize) -> FieldCell {
        self.field[x][y]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: FieldCell) {
        self.field[x][y] = value;
    }

    pub fn will_ship_survive_after_shot(&self, x: usize, y: usize) -> bool {
        let x = x as isize;
        let y = y as isize;

        return walk(self, x, y, 1, 0) ||
            walk(self, x, y, -1, 0) ||
            walk(self, x, y, 0, 1) ||
            walk(self, x, y, 0, -1);


        fn walk(me: &PlayerField, x: isize, y: isize, xdir: isize, ydir: isize) -> bool {
            if me.is_out_of_bounds(x + xdir, y + ydir) {
                return false;
            }

            match me.at((x + xdir) as usize, (y + ydir) as usize) {
                FieldCell::Ship => true,
                FieldCell::Hit => walk(me, x + xdir, y + ydir, xdir, ydir),
                _ => false,
            }
        }
    }

    pub fn mark_dead(&mut self, x: usize, y: usize) {
        mark_dead_recursive(self, x as isize, y as isize);

        fn mark_dead_recursive(me: &mut PlayerField, x: isize, y: isize) {
            if me.is_out_of_bounds(x, y) {
                return;
            }

            let cell = me.at(x as usize, y as usize);

            if cell == FieldCell::Ship || cell == FieldCell::Hit {
                me.field[x as usize][y as usize] = FieldCell::Dead;
                mark_dead_recursive(me, x - 1, y);
                mark_dead_recursive(me, x + 1, y);
                mark_dead_recursive(me, x, y + 1);
                mark_dead_recursive(me, x, y - 1);
            }
        }
    }

    pub fn mark_hit(&mut self, x: usize, y: usize) {
        self.set(x, y, FieldCell::Hit);
    }

    pub fn mark_enemy_miss(&mut self, x: usize, y: usize) {
        self.set(x, y, FieldCell::EnemyMiss);
    }

    pub fn is_ship_on(&self, x: usize, y: usize) -> bool {
        self.at(x, y) == FieldCell::Ship
    }

    pub fn place_ship(&mut self, ship: &Ship) {
        debug_assert!(self.can_place(ship).is_ok());

        match ship.rotation {
            Rotation::Horizontal => for i in 0..ship.length {
                self.field[ship.x + i][ship.y] = FieldCell::Ship 
            },
            Rotation::Vertical => for i in 0..ship.length { 
                self.field[ship.x][ship.y + i] = FieldCell::Ship 
            },    
        }
    }

    pub fn can_place(&self, ship: &Ship) -> Result<(), ShipPlacementError> {
        match ship.rotation {
            Rotation::Horizontal => for i in 0..ship.length {
                if let Err(error) = self.can_place_on(ship.x + i, ship.y) {
                    return Err(error);
                }
            },
            Rotation::Vertical => for i in 0..ship.length { 
                if let Err(error) = self.can_place_on(ship.x, ship.y + i) {
                    return Err(error);
                }
            },    
        }

        Ok(())
    }

    fn can_place_on(&self, x: usize, y: usize) -> Result<(), ShipPlacementError> {
        if self.out_of_bounds(x) {
            return Err(ShipPlacementError::OutOfBoundsX);
        }
        if self.out_of_bounds(y) {
            return Err(ShipPlacementError::OutOfBoundsY);
        }

        for dy in -1..=1isize {
            for dx in -1..=1isize {
                let checked_x = if dx == -1 { sub_one(x) } else if dx == 0 { Some(x) } else { add_one(x, self.size) };
                let checked_y = if dy == -1 { sub_one(y) } else if dy == 0 { Some(y) } else { add_one(y, self.size) };

                let checked_x = match checked_x {
                    Some(value) => value,
                    None => { continue; }
                };
                let checked_y = match checked_y {
                    Some(value) => value,
                    None => { continue; }
                };

                if self.is_ship_on(checked_x, checked_y) {
                    return Err(ShipPlacementError::TouchesWithShip(checked_x, checked_y));
                }
            }
        }

        Ok(())
    }

    fn out_of_bounds(&self, coordinate: usize) -> bool {
        coordinate >= self.size
    }

    fn is_out_of_bounds(&self, x: isize, y: isize) -> bool {
        let size = self.size as isize;
        x < 0 || y < 0 || x >= size || y >= size
    }
}

fn sub_one(coordinate: usize) -> Option<usize> {
    if coordinate == 0 { None } else { Some(coordinate - 1) }
}
fn add_one(coordinate: usize, bounds: usize) -> Option<usize> {
    if coordinate == bounds - 1 { None } else { Some(coordinate + 1) }
}

pub enum ShipPlacementError {
    OutOfBoundsX,
    OutOfBoundsY,
    TouchesWithShip(usize, usize),
    NoShipsOfLengthLeft(usize),
}

impl Display for ShipPlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipPlacementError::OutOfBoundsX => write!(f, "x was out of bounds! Should be in range 0..{FIELD_SIZE}"),
            ShipPlacementError::OutOfBoundsY => write!(f, "y was out of bounds! Should be in range {FIRST_LETTER}..{LAST_LETTER}"),
            ShipPlacementError::NoShipsOfLengthLeft(length) => write!(f, "no ships of length {length} left"),
            ShipPlacementError::TouchesWithShip(other_x, other_y) => {
                let other_x = other_x + 1;
                let other_y = conversions::usize_to_coordinate(*other_y);
                write!(f, "collision with another ship at {other_y}{other_x}")
            },
        }
    }
}

