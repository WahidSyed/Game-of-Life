
use patterns::Pattern;

/* Blinker */
#[derive(Debug)]
pub struct Blinker {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Blinker {
    const SIZE: u32 = 3;
}

impl Pattern for Blinker {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Blinker::SIZE }
    fn height(&self) -> u32 { Blinker::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Blinker {
        let mut blinker = Blinker {
            x, y,
            structure: vec![false; (Blinker::SIZE * Blinker::SIZE) as usize],
        };
        blinker.structure = vec![
            false, true, false,
            false, true, false,
            false, true, false,
        ];
        blinker
    }
}


/* Toad */
#[derive(Debug)]
pub struct Toad {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Toad {
    const SIZE: u32 = 4;
}

impl Pattern for Toad {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Toad::SIZE }
    fn height(&self) -> u32 { Toad::SIZE }
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

/* Beacon */
#[derive(Debug)]
pub struct Beacon {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Beacon {
    const SIZE: u32 = 4;
}

impl Pattern for Beacon {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Beacon::SIZE }
    fn height(&self) -> u32 { Beacon::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Beacon {
        let mut beacon = Beacon {
            x, y,
            structure: vec![false; (Beacon::SIZE * Beacon::SIZE) as usize],
        };
        beacon.structure = vec![
            true,true,false,false,
            true,false,false,false,
            false,false,false,true,
            false,false,true,true,
        ];
        beacon
    }
}

/* Pulsar */
#[derive(Debug)]
pub struct Pulsar {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Pulsar {
    const SIZE: u32 = 15;
}

impl Pattern for Pulsar {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Pulsar::SIZE }
    fn height(&self) -> u32 { Pulsar::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Pulsar {
        let mut pulsar = Pulsar {
            x, y,
            structure: vec![false; (Pulsar::SIZE * Pulsar::SIZE) as usize],
        };
        pulsar.structure = vec![
            false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,false,true,true,true,false,false,false,true,true,true,false,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,false,false,true,true,true,false,false,false,true,true,true,false,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,false,true,true,true,false,false,false,true,true,true,false,false,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,true,false,false,false,false,true,false,true,false,false,false,false,true,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,false,true,true,true,false,false,false,true,true,true,false,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,
        ];
        pulsar
    }
}

/* Pentadecathlon */
#[derive(Debug)]
pub struct Pentadecathlon {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Pentadecathlon {
    const WIDTH: u32 = 9;
    const HEIGHT: u32 = 16;
}

impl Pattern for Pentadecathlon {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Pentadecathlon::WIDTH }
    fn height(&self) -> u32 { Pentadecathlon::HEIGHT }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> Pentadecathlon {
        let mut pentadecathlon = Pentadecathlon {
            x, y,
            structure: vec![false; (Pentadecathlon::WIDTH * Pentadecathlon::HEIGHT) as usize],
        };
        pentadecathlon.structure = vec![
            false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,true,false,true,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,true,false,true,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,true,false,false,false,false,
            false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,
        ];
        pentadecathlon
    }
}



/* KoksGalaxy */
#[derive(Debug)]
pub struct KoksGalaxy {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl KoksGalaxy {
    const SIZE: u32 = 13;
}

impl Pattern for KoksGalaxy {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { KoksGalaxy::SIZE }
    fn height(&self) -> u32 { KoksGalaxy::SIZE }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> KoksGalaxy {
        let mut koks_galaxy = KoksGalaxy {
            x, y,
            structure: vec![false; (KoksGalaxy::SIZE * KoksGalaxy::SIZE) as usize],
        };
        koks_galaxy.structure = vec![
            false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,true,true,false,true,true,true,true,true,true,false,false,
            false,false,true,true,false,true,true,true,true,true,true,false,false,
            false,false,true,true,false,false,false,false,false,false,false,false,false,
            false,false,true,true,false,false,false,false,false,true,true,false,false,
            false,false,true,true,false,false,false,false,false,true,true,false,false,
            false,false,true,true,false,false,false,false,false,true,true,false,false,
            false,false,false,false,false,false,false,false,false,true,true,false,false,
            false,false,true,true,true,true,true,true,false,true,true,false,false,
            false,false,true,true,true,true,true,true,false,true,true,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,false,false,false,false,false,false,
        ];
        koks_galaxy
    }
}

