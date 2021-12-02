use std::fs::OpenOptions;
use std::io::{self, BufRead};

fn main() {
    let file = OpenOptions::new().read(true).open("input.txt").unwrap();
    let buffer = io::BufReader::new(file);

    let mut coords: [u32; 2] = [0, 0];
    let mut coords_with_aim: [u32; 2] = [0, 0];
    let mut aim: u32 = 0;

    for opt_line in buffer.lines() {
        let line = opt_line.unwrap();
        let arguments: Vec<&str> = line.split(' ').collect();
        let command = arguments[0];
        let unit: u32 = arguments[1].parse().unwrap();

        match command {
            "forward" => {
                coords[0] += unit;
                coords_with_aim[0] += unit;
                coords_with_aim[1] += aim*unit;
            },
            "down" => {
                coords[1] += unit;
                aim += unit;
            },
            "up" => {
                coords[1] -= unit;
                aim -= unit;
            },
            _ => panic!("invalid command"),
        }
    }

    println!("Final coordinates: ({}, {})", coords[0], coords[1]);
    println!("Final coordinates product: {}", coords[0] * coords[1]);
    println!("Final coordinates with aim: ({}, {})", coords_with_aim[0], coords_with_aim[1]);
    println!("Final coordinates with aim product: {}", coords_with_aim[0] * coords_with_aim[1]);
}
