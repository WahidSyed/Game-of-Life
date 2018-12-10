use pattern::Pattern;
use decoder::RleDecoder;
use config::*;

pub struct Life {
    board: [u64; AH],
}

impl Life {

    pub fn new() -> Self {
        Life { board: [0; AH] }
    }

    fn validate(x: u64, y: u64) {
        if x > GW || y > GH {
            panic!("counldn't set ({}, {}): out of bounds", x, y);
        }
    }

    fn position(x: u64, y: u64) -> (usize, u64) {
        Life::validate(x, y);
        let i: usize = (y * RW as u64 + x / AW as u64) as usize;
        let pos: u64 = x % AW as u64;
        (i, pos)
    }

    fn seed(&mut self, x: u64, y: u64) {
        let (i, pos) = Life::position(x, y);
        self.board[i] |= 1<<pos;
    }

    fn set_bit(&mut self, a: &mut [u64; AH], x: u64, y: u64) {
        let (i, pos) = Life::position(x, y);
        a[i] |= 1<<pos;
    }

    fn clear_bit(&mut self, a: &mut [u64; AH], x: u64, y: u64) {
        let (i, pos) = Life::position(x, y);
        a[i] &= !(1<<pos);
    }

    pub fn test_bit(&mut self, x: u64, y: u64) -> bool {
        let (i, pos) = Life::position(x, y);
        self.board[i] & (1<<pos) != 0
    }

    fn live_neighbours(&mut self, x: i64, y: i64) -> u8 {
        let mut n = 0;
        for row in y-1..y+2 {
            for col in x-1..x+2 {
                if (row >= 0 && row < GH as i64)
                && (col >= 0 && col < GW as i64)
                && !(row == y && col == x)
                && self.test_bit(col as u64, row as u64) {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn next_gen(&mut self) {
        let mut next = self.board.clone();
        for y in 0..GH {
            for x in 0..GW {
                let n = self.live_neighbours(x as i64, y as i64);
                match self.test_bit(x, y) {
                    true => {
                        if n > 1 && n < 4 {
                            self.set_bit(&mut next, x, y);
                        } else {
                            self.clear_bit(&mut next, x, y);
                        }
                    },
                    false => if n == 3 {
                        self.set_bit(&mut next, x, y);
                    },
                }
            }
        }
        self.board = next;
    }

    pub fn add_pattern(&mut self, p: Pattern, (offset_x, offset_y): (u64, u64)) {
        p.print_meta();
        p.print_pattern();
        println!();
        
        for y in 0..p.height {
            for x in 0..p.width {
                match p.structure[RleDecoder::index(y, x, p.width)] {
                    1 => self.seed(y+offset_y, x+offset_x),
                    _ => (),
                }
            }
        }

    }

}



