#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Rotation {
    Horizontal,
    Vertical
}

pub struct Ship {
    pub length: usize,
    pub x: usize,
    pub y: usize,
    pub rotation: Rotation,
}
