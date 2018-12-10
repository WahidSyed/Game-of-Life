extern crate piston_window;
extern crate image as im;
extern crate find_folder;
use piston_window::*;

mod pattern;
mod decoder;
mod life;
mod fps_counter;
mod config;

use life::Life;
use fps_counter::FPSCounter;
use config::*;

/*

Life. Optimized for speed using fixed sized bit arrays
Todo: Multi-threaded next gen calculation

*/

fn main() {
    let p = decoder::get_rle_pattern(String::from("./patterns"));
    let mut life = Life::new();
    life.add_pattern(p, (20, 20));

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

        life.next_gen();

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                life.next_gen();
            }
        };

    }
}


