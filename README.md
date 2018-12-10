# Conway's Game of Life

<p align="left">
  <img src="./demo.png" width="300" >
</p>

My implementation of Conway's Game of Life in Rust. It uses a Piston game engine to render the graphics window.

GOL theory: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life



### Patterns


### Performance
Implementation was done using a fixed 64 bit array, the program is able to render with great speed.
Changing the game size is done by modifying config.rs since array sizes in Rust must be known at compile time. This particular array implementation was chosen over dynamic implementations like vectors to yield maximum performance.
Note that the array size must be dividisble by 64^n.

Size | FPS
-----|----
256x256 | 60fps
512x512 | ~50fps
1024x1024 | ~24fps

### Usage
```
git clone https://github.com/jdansev/game-of-life
cd game-of-life
sh download-patterns.sh
cargo run --release
```
