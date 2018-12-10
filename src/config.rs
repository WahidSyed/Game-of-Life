
// game width
pub const GW: u64 = 64*32;
// game height
pub const GH: u64 = 64*32;

// window width
pub const WW: u64 = 600;
// window height
pub const WH: u64 = 600;

// array width (fixed u64 size)
pub const AW: usize = 64;
// array height
pub const AH: usize = (GW / AW as u64 * GH) as usize;

// block size
pub const BS: f64 = (WW as f64 / GW as f64);
// row width
pub const RW: usize = (GW / AW as u64) as usize;

// color definitions
pub const GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
