
use patterns::Pattern;

/* Toad */
#[derive(Debug)]
pub struct Toad {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Toad {
    const SIZE: u8 = 4;
}

impl Pattern for Toad {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn size(&self) -> u8 { Toad::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Toad {
        let mut toad = Toad {
            x, y,
            structure: vec![false; (Toad::SIZE * Toad::SIZE) as usize],
        };
        toad.structure = vec![
            false, false, true, false,
            true, false, false, true,
            true, false, false, true,
            false, true, false, false,
        ];
        toad
    }
}