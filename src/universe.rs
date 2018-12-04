
use patterns::Pattern;

use std::time::Duration;
use std::thread;

const TICK_RATE: u64 = 10; // ms

#[derive(Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

impl Universe {

    pub fn cells(&self) -> Vec<bool> {
        self.cells.clone()
    }

    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width, height,
            cells: vec![false; (width*height) as usize],
        }
    }

    pub fn get_index(&self, row: i32, col: i32) -> usize {
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

    pub fn next_generation(&mut self) {
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
        thread::sleep(Duration::from_millis(TICK_RATE))
    }

    pub fn add_pattern<P: Pattern>(&mut self, p: &mut P) {
        for x in 0..p.height() {
            for y in 0..p.width() {
                let i1 = p.get_index(x as i32, y as i32);
                let i2 = self.get_index((x + p.x() as u32) as i32, (y + p.y() as u32) as i32);
                self.cells[i2] = p.structure()[i1];
            }
        }
    }

}


