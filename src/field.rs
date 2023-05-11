use super::{Ship, Rotation};

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum FieldCell {
    Empty,
    Ship
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
        self.field[x][y] = marker;
    }

    pub fn at(&self, x: usize, y: usize) -> CheckedCell {
        assert!(x < self.size && y < self.size);
        self.field[x][y]
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

    pub fn at(&self, x: usize, y: usize) -> FieldCell {
        assert!(x < self.size && y < self.size);
        self.field[x][y]
    }

    pub fn is_ship_on(&self, x: isize, y: isize) -> bool {
        if self.is_out_of_bounds(x, y) {
            return false;
        }

        self.field[x as usize][y as usize] == FieldCell::Ship
    }

    pub fn place_ship(&mut self, ship: &Ship) -> bool {
        debug_assert!(self.can_place(ship));

        match ship.rotation {
            Rotation::Horizontal => for i in 0..ship.length {
                self.field[ship.x + i][ship.y] = FieldCell::Ship 
            },
            Rotation::Vertical => for i in 0..ship.length { 
                self.field[ship.x][ship.y + i] = FieldCell::Ship 
            },    
        }

        true
    }

    pub fn can_place(&self, ship: &Ship) -> bool {
        match ship.rotation {
            Rotation::Horizontal => for i in 0..ship.length {
                if !self.can_place_on(ship.x + i, ship.y) {
                    return false;
                }
            },
            Rotation::Vertical => for i in 0..ship.length { 
                if !self.can_place_on(ship.x, ship.y + i) {
                    return false;
                }
            },    
        }

        true
    }

    fn can_place_on(&self, x: usize, y: usize) -> bool {
        if self.is_out_of_bounds(x as isize, y as isize) {
            return false;
        }

        for dy in -1..=1isize {
            for dx in -1..=1isize {
                let checked_x = x as isize + dx;
                let checked_y = y as isize + dy;

                if self.is_ship_on(checked_x, checked_y) {
                    return false;
                }
            }
        }

        true
    }

    fn is_out_of_bounds(&self, x: isize, y: isize) -> bool {
        let size = self.size as isize;
        x < 0 || y < 0 || x >= size || y >= size
    }
}
