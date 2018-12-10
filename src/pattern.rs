
use decoder::RleDecoder;

#[derive(Debug)]
pub struct Pattern {
    name: String,
    found: String,
    description: String,
    link: String,
    pub width: u64,
    pub height: u64,
    pub structure: Vec<u64>,
}

impl Pattern {
    pub fn new(rle: &str) -> Self {
        let (name, found, description, link, width, height, structure) = RleDecoder::decode(rle);
        Pattern { name, found, description, link, width, height, structure }
    }

    pub fn default() -> Self {
        let (name, found, description, link, width, height, structure) = RleDecoder::default();
        Pattern { name, found, description, link, width, height, structure }
    }

    pub fn print_meta(&self) {
        println!("\nPattern Name: {}", self.name);
        println!("Found: {}", self.found);
        println!("Description: {}", self.description);
        println!("Link: {}", self.link);
        println!("Dimensions: {} x {}", self.width, self.height);
    }

    pub fn print_pattern(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.structure[RleDecoder::index(y, x, self.width)] {
                    0 => print!(". "),
                    1 => print!("* "),
                    _ => (),
                }
            }
            println!();
        }
    }

}