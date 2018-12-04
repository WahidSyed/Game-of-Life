

const ALIVE: char = '*';
const DEAD: char = '.';

pub fn hello() {
    println!("hello!");
}

pub trait Pattern {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn new(x: i32, y: i32) -> Self;
    fn size(&self) -> u8;
    fn structure(&mut self) -> Vec<bool>;

    fn get_index(&self, row: i32, col: i32) -> usize {
        let h = self.size() as i32;
        let w = self.size() as i32;
        let r = ((row % h) + h) % h;
        let c = ((col % w) + w) % w;
        (r * w + c) as usize
    }

    fn print(&mut self) {
        for r in 0..self.size() {
            for c in 0..self.size() {
                match self.structure()[self.get_index(r as i32, c as i32)] {
                    true => print!("{} ", ALIVE),
                    false => print!("{} ", DEAD),
                }
            }
            println!();
        }
    }
}