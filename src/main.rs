extern crate piston_window;
extern crate image as im;
extern crate find_folder;
use piston_window::*;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

// Life. Optimized for speed using fixed sized bit arrays
// Todo: Multi-threaded next gen calculation

// game width
const GW: u64 = 64*4;
// game height
const GH: u64 = 64*4;
// window width
const WW: u64 = 600;
// window height
const WH: u64 = 600;
// block size
const BS: f64 = (WW as f64 / GW as f64);
// array width (fixed u64 size)
const AW: usize = 64;
// array height
const AH: usize = (GW / AW as u64 * GH) as usize;
// row width
const RW: usize = (GW / AW as u64) as usize;
// color definitions
const GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];


struct Life {
    board: [u64; AH],
}

impl Life {

    fn new() -> Self {
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

    fn seed(&mut self,  x: u64, y: u64) {
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

    fn test_bit(&mut self, x: u64, y: u64) -> bool {
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

    fn next_gen(&mut self) {
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

}



pub struct FPSCounter {
    last_second_frames: VecDeque<Instant>
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);
        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }
        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}


fn main() {
    let mut life = Life::new();

    // acorn pattern
    life.seed(101, 100);
    life.seed(103, 101);
    life.seed(104, 102);
    life.seed(105, 102);
    life.seed(106, 102);
    life.seed(100, 102);
    life.seed(101, 102);


    let opengl = OpenGL::V3_2;
    let (width, height) = (WW as u32, WH as u32);
    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let canvas = im::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("Meslo-LG.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let mut fps_counter = FPSCounter::new();

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&e, |c, g| {

                clear(DARK, g);
                image(&texture, c.transform, g);

                for hi in 0..GH {
                    let hp = hi as f64 * BS;
                    for wi in 0..GW {
                        let wp = wi as f64 * BS;
                        if life.test_bit(hi, wi) {
                            rectangle(
                                WHITE, [wp as f64, hp as f64, BS, BS],
                                c.transform, g
                            );
                        }
                    }
                }
                life.next_gen();

                let frame_rate = format!("{}fps", &fps_counter.tick().to_string());
                let fps_transform = c.transform.trans(10.0, WH as f64 - 10.0);

                text::Text::new_color(GRAY, 16).draw(
                    &frame_rate,
                    &mut glyphs,
                    &c.draw_state,
                    fps_transform, g
                ).unwrap();

            });
        }

    }
}


