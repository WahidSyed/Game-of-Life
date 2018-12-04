
mod patterns;
use patterns::Pattern;

mod spaceships;
use spaceships::Glider;

mod oscillators;
use oscillators::Toad;

use std::time::Duration;
use std::thread;

const WIDTH: u32 = 80;
const HEIGHT: u32 = 80;

const ALIVE: char = '*';
const DEAD: char = '.';
const MILLIS: u64 = 100;


#[derive(Debug)]
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

impl Universe {

    fn new(width: u32, height: u32) -> Universe {
        Universe {
            width, height,
            cells: vec![false; (width*height) as usize],
        }
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        let h = self.height as i32;
        let w = self.width as i32;
        let r = ((row % h) + h) % h;
        let c = ((col % w) + w) % w;
        (r * w + c) as usize
    }

    fn live_neighbours(&self, row: i32, col: i32) -> u8 {
        let mut count = 0;
        let h = self.height as i32;
        let w = self.width as i32;
        for r in row-1..row+2 {
            for c in col-1..col+2 {
                if (r >= 0 && r < h)
                && (c >= 0 && c < w)
                && !(r == row && c == col)
                && self.cells[self.get_index(r as i32, c as i32)]
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn next_generation(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {

                let index = self.get_index(row as i32, col as i32);
                let n = self.live_neighbours(row as i32, col as i32);

                match self.cells[index] {
                    true => {
                        if n > 1 && n < 4 {
                            next[index] = true;
                        } else {
                            next[index] = false;
                        }
                    },
                    false => {
                        if n == 3 {
                            next[index] = true;
                        }
                    },
                }

            }
        }
        self.cells = next;
        thread::sleep(Duration::from_millis(MILLIS))
    }

    fn print(&mut self) {
        for r in 0..self.height {
            for c in 0..self.width {

                match self.cells[self.get_index(r as i32, c as i32)] {
                    true => print!("{} ", ALIVE),
                    false => print!("{} ", DEAD),
                }

            }
            println!();
        }
    }

    fn iterate(&mut self, n: u32) {
        self.print();
        println!();
        for _i in 0..n {
            self.next_generation();
            self.print();
            println!();
        }
    }

    fn add_pattern<P: Pattern>(&mut self, p: &mut P) {
        for x in 0..p.size() {
            for y in 0..p.size() {
                let i1 = p.get_index(x as i32, y as i32);
                let i2 = self.get_index((x + p.x() as u8) as i32, (y + p.y() as u8) as i32);
                self.cells[i2] = p.structure()[i1];
            }
        }
    }

}




fn main() {
    let mut universe = Universe::new(WIDTH, HEIGHT);

    let mut glider: Glider = Pattern::new(5, 7);
    let mut toad1: Toad = Pattern::new(30, 30);
    let mut toad2: Toad = Pattern::new(15, 27);

    universe.add_pattern(&mut toad1);
    universe.add_pattern(&mut toad2);
    universe.add_pattern(&mut glider);

    universe.iterate(300);
}
