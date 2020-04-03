## Conway's Game of Life

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

#1482 - Pufferfish Breeder (set game width/height to 64*16)  
#247 - Turing Machine (set game width/height to 64*32)  
#2056 - Gosper Glider Gun (set game width/height to 64*4)  


### Patterns
Patterns are downloaded with the script `download-patterns.sh` which come as 1896 .rle (run-length encoded) files. This must be done before running. My program includes a decoder that can parse this format and use it to seed the grid. The full list of patterns and their descriptions can be found [here](http://www.conwaylife.com/wiki/Category:Patterns).

### Performance
The grid is stored in an array of 64 bit unsigned integers, with each cell occupying 1 bit. This representation was chosen over dynamic data structures like vectors for maximum performance. The tradeoff is that changing the grid size requires modifying the config.rs file (located in the src folder), since the size of static arrays must be known by Rust at compile time. Also note that the game size must be of the form 64 * 2^n.

And here are a few benchmarks I ran at different sized grids (2.4 GHz Intel Core i5):

Size | Cells | FPS | s/Generation
-----|-------|-----|---------------
256x256 | 65,536 | 60fps+ | 0.016s
512x512 | 262,144 | ~50fps | 0.020s
1024x1024 | 1,048,576 | ~24fps | 0.042s
2048x2048 | 4,194,304 | ~2fps | 0.500s
