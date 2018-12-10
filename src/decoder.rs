use std::fs::File;
use std::io::prelude::*;
use std::fs::{self};
use std::io::{self, Write};

use pattern::Pattern;


pub struct RleDecoder {}

impl RleDecoder {

    pub fn default() -> (String, String, String, String, u64, u64, Vec<u64>) {
        return RleDecoder::decode("#C no data\nx = 0, y = 0\n!");
    }

    pub fn index(r: u64, c: u64, w: u64) -> usize {
        (r * w + c) as usize
    }

    pub fn decode(rle: &str) -> (String, String, String, String, u64, u64, Vec<u64>) {
        let mut x: &str = "";
        let mut y: &str = "";
        let mut name: String = String::new();
        let mut found: String = String::new();
        let mut description: String = String::new();
        let mut link: String = String::new();

        let lines = rle.split("\n").collect::<Vec<&str>>();
        let mut num_comments = 0;

        // get dimensions
        for i in 0..lines.len() {
            let line = lines[i].trim();
            let metadata: String = line.chars().take(2).collect();

            match metadata.as_ref() {
                "#N" => {
                    name = line.chars().skip(2).collect::<String>().trim().to_string();
                    continue;
                },
                "#O" => {
                    found = line.chars().skip(2).collect::<String>().trim().to_string();
                    continue;
                },
                "#C" if num_comments == 0 => {
                    description = line.chars().skip(2).collect::<String>().trim().to_string();
                    num_comments += 1;
                    continue;
                },
                "#C" if num_comments == 1 => {
                    link = line.chars().skip(2).collect::<String>().trim().to_string();
                    num_comments += 1;
                    continue;
                },
                "#C" => {
                    continue;
                },
                "x " => {
                    // header
                    let header = line.split(",");
                    let header = header.collect::<Vec<&str>>();
                    x = header[0].trim().split("=").collect::<Vec<&str>>()[1].trim();
                    y = header[1].trim().split("=").collect::<Vec<&str>>()[1].trim();
                    break;
                }
                _ => {
                    return RleDecoder::decode("#C no data\nx = 0, y = 0\n!");
                },
            }
            
        }

        let x = x.parse::<u64>().unwrap();
        let y = y.parse::<u64>().unwrap();

        let mut first_line:bool = false;
        let mut structure: Vec<u64> = vec![0; x as usize * y as usize];
        let mut c_x: u64 = 0;
        let mut c_y: u64 = 0;

        // decode
        for i in 0..lines.len() {
            let line = lines[i].trim();
            let ch: String = line.chars().take(1).collect();

            if ch == "#" {
                continue;
            } else if !first_line {
                first_line = true;
                continue;
            }

            let line_chars: Vec<char> = line.chars().collect();
            let mut j = 0;
            while j < line_chars.len() {
                let mut ch = line_chars[j];

                if ch == '!' { break; }

                let mut run_length: u64 = 0;
                while ch.is_digit(10) && j < line.chars().count() {
                    run_length = run_length * 10 + ch.to_digit(10).unwrap() as u64;
                    j += 1;
                    ch = line_chars[j];
                }

                match ch {
                    // next line
                    '$' => {
                        if run_length == 0 {
                            c_y += 1;
                        } else if run_length > 0 {
                            let c_y_temp = c_y;
                            while c_y < c_y_temp + run_length {
                                c_y += 1;
                            }
                        }
                        c_x = 0;
                    },
                    // dead cell
                    'b' => {
                        if run_length == 0 {
                            let index = RleDecoder::index(c_y, c_x, x);
                            structure[index] = 0;
                            c_x += 1;
                        } else if run_length > 0 {
                            let c_x_temp = c_x;
                            while c_x < c_x_temp + run_length {
                                let index = RleDecoder::index(c_y, c_x, x);
                                structure[index] = 0;
                                c_x += 1;
                            }
                        }
                    },
                    // alive cell
                    'o' => {
                        if run_length == 0 {
                            let index = RleDecoder::index(c_y, c_x, x);
                            structure[index] = 1;
                            c_x += 1;
                        } else if run_length > 0 {
                            let c_x_temp = c_x;
                            while c_x < c_x_temp + run_length {
                                let index = RleDecoder::index(c_y, c_x, x);
                                structure[index] = 1;
                                c_x += 1;
                            }
                        }
                    },
                    _ => (),
                }
                j += 1;
            }

        }
        (name, found, description, link, x, y, structure)
    }

}


fn parse_pattern(path: &String) -> Pattern {
    let rle_path = "./patterns/".to_owned() + path.as_str();
    let mut f = File::open(&rle_path).expect("file not found");
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Result::Ok(_) => (),
        Result::Err(_) => {
            contents = String::from("#C no data\nx = 0, y = 0\n!");
        },
    }
    Pattern::new(&contents.to_string())
}

pub fn get_rle_pattern(dir_path: String) -> Pattern {

    let rle_paths =
    fs::read_dir(dir_path).unwrap()
    .filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
    )
    }).collect::<Vec<String>>();

    for (i, rle_path) in rle_paths.iter().enumerate() {
        println!("[{}] {}", i, rle_path);
    }

    print!("{} patterns\n> ", rle_paths.len());
    match io::stdout().flush() {
        _ => (),
    }

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    
    match trimmed.parse::<usize>() {
        Ok(i) => {
            println!("your integer input: {}", i);
            return parse_pattern(&rle_paths[i]);
        },
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    Pattern::default()
}
