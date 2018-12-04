
use patterns::Pattern;

/* Glider */
#[derive(Debug)]
pub struct Glider {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl Glider {
    const SIZE: u32 = 3;
}

impl Pattern for Glider {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { Glider::SIZE }
    fn height(&self) -> u32 { Glider::SIZE }
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

/* Edge Repair Spaceship 1 */
#[derive(Debug)]
pub struct EdgeRepair1 {
    x: i32,
    y: i32,
    structure: Vec<bool>,
}

impl EdgeRepair1 {
    const WIDTH: u32 = 16;
    const HEIGHT: u32 = 7;
}

impl Pattern for EdgeRepair1 {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> u32 { EdgeRepair1::WIDTH }
    fn height(&self) -> u32 { EdgeRepair1::HEIGHT }
    fn structure(&mut self) -> Vec<bool> { self.structure.clone() }

    fn new(x: i32, y: i32) -> EdgeRepair1 {
        let mut edge_repair1 = EdgeRepair1 {
            x, y,
            structure: vec![false; (EdgeRepair1::WIDTH * EdgeRepair1::HEIGHT) as usize],
        };
        edge_repair1.structure = vec![
            false,false,false,false,false,false,false,false,true,false,false,false,false,false,false,false,
            false,false,false,false,false,false,false,true,true,true,true,false,false,false,false,false,
            false,false,true,false,false,false,true,false,false,false,true,true,false,true,true,false,
            false,true,true,true,true,false,false,false,false,false,true,false,false,true,true,false,
            true,false,false,false,true,false,false,false,false,false,false,false,true,false,false,true,
            false,true,false,true,false,false,true,false,false,false,false,false,false,false,false,false,
            false,false,false,false,false,true,false,false,false,false,false,false,false,false,false,false,
        ];

        /*
            0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,
            0,0,1,0,0,0,1,0,0,0,1,1,0,1,1,0,
            0,1,1,1,1,0,0,0,0,0,1,0,0,1,1,0,
            1,0,0,0,1,0,0,0,0,0,0,0,1,0,0,1,
            0,1,0,1,0,0,1,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,
        */

        edge_repair1
    }
}