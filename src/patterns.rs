

pub trait Pattern {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn new(x: i32, y: i32) -> Self;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn structure(&mut self) -> Vec<bool>;

    fn get_index(&self, row: i32, col: i32) -> usize {
        let h = self.height() as i32;
        let w = self.width() as i32;
        let r = ((row % h) + h) % h;
        let c = ((col % w) + w) % w;
        (r * w + c) as usize
    }

}