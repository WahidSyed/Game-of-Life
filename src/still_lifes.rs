
use patterns::Pattern;

/* Block */
pub struct Block {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Block {
    const SIZE: u32 = 2;
}

impl Pattern for Block {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Block::SIZE }
    fn height(&self) -> u32 { Block::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Block {
        let mut block = Block {
            x, y,
            structure: vec![false; (Block::SIZE * Block::SIZE) as usize],
        };
        block.structure = vec![
            true, true,
            true, true,
        ];
        block
    }
}


/* Beehive */
pub struct Beehive {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Beehive {
    const WIDTH: u32 = 4;
    const HEIGHT: u32 = 3;
}

impl Pattern for Beehive {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Beehive::WIDTH }
    fn height(&self) -> u32 { Beehive::HEIGHT }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Beehive {
        let mut beehive = Beehive {
            x, y,
            structure: vec![false; (Beehive::WIDTH * Beehive::HEIGHT) as usize],
        };
        beehive.structure = vec![
            false,true,true,false,
            true,false,false,true,
            false,true,true,false,
        ];
        beehive
    }
}



/* Loaf */
pub struct Loaf {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Loaf {
    const SIZE: u32 = 4;
}

impl Pattern for Loaf {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Loaf::SIZE }
    fn height(&self) -> u32 { Loaf::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Loaf {
        let mut loaf = Loaf {
            x, y,
            structure: vec![false; (Loaf::SIZE * Loaf::SIZE) as usize],
        };
        loaf.structure = vec![
            false,true,true,false,
            true,false,false,true,
            false,true,false,true,
            false,false,true,false,
        ];
        loaf
    }
}


/* Boat */
pub struct Boat {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Boat {
    const SIZE: u32 = 3;
}

impl Pattern for Boat {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Boat::SIZE }
    fn height(&self) -> u32 { Boat::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Boat {
        let mut boat = Boat {
            x, y,
            structure: vec![false; (Boat::SIZE * Boat::SIZE) as usize],
        };
        boat.structure = vec![
            true,true,false,
            true,false,true,
            false,true,false,
        ];
        boat
    }
}


/* Tub */
pub struct Tub {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Tub {
    const SIZE: u32 = 3;
}

impl Pattern for Tub {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Tub::SIZE }
    fn height(&self) -> u32 { Tub::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Tub {
        let mut tub = Tub {
            x, y,
            structure: vec![false; (Tub::SIZE * Tub::SIZE) as usize],
        };
        tub.structure = vec![
            false,true,false,
            true,false,true,
            false,true,false,
        ];
        tub
    }
}