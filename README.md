# Conway's Game of Life

<p align="left">
  <img src="./turingmachine.png" width="400" >
  <br/>Pattern: Turing Machine
</p>




*Conway's Game of Life* is a fascinating cellular automaton. This is my implementation of the game in Rust, built on a Piston game engine. Theory: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

### Usage
```
git clone https://github.com/jdansev/game-of-life
cd game-of-life
sh download-patterns.sh
cargo run --release
```

### Patterns
Patterns are downloaded with the script `download-patterns.sh` which come as 1896 .rle (run-length encoded) files. This must be done before running. My program includes a decoder that can parse this format and use it to seed the game board.

### Performance
The grid is stored in an array of 64 bit unsigned integers, with each cell occupying 1 bit. This representation was chosen over dynamic data structures like vectors for maximum performance. The tradeoff is that changing the game size requires modifying the config.rs, since the size of static arrays must be known by Rust at compile time.
Note that the game size must be in the form of 64 * 2^n.

And here are a few benchmarks I ran at different sizes (2.4 GHz Intel Core i5):

Size | FPS | ms/Generation
-----|-----|----------
256x256 (65,536 cells) | 60fps+ | 0.016ms
512x512 (262,144 cells) | ~50fps | 0.020ms
1024x1024 (1,048,576 cells) | ~24fps | 0.042ms
2048x2048 (4,194,304 cells) | ~2fps | 0.50ms
