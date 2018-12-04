
use patterns::Pattern;

/* Glider */
#[derive(Debug)]
pub struct Glider {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Glider {
    const SIZE: u8 = 3;
}

impl Pattern for Glider {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn size(&self) -> u8 { Glider::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Glider {
        let mut glider = Glider {
            x, y,
            structure: vec![false; (Glider::SIZE * Glider::SIZE) as usize],
        };
        glider.structure = vec![
            false, true, false,
            false, false, true,
            true, true, true,
        ];
        glider
    }
}