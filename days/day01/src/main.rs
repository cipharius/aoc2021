use std::fs::OpenOptions;
use std::io::{self, BufRead};

fn main() {
    let file = OpenOptions::new().read(true).open("input.txt").unwrap();
    let buffer = io::BufReader::new(file);

    let mut last_number: Option<u32> = None;
    let mut last_window_sum: Option<u32> = None;
    let mut window: [u32; 3] = [0; 3];
    let mut larger_count = 0;
    let mut larger_window_count = 0;

    for (i, opt_line) in buffer.lines().enumerate() {
        let number: u32 = opt_line.unwrap().parse().unwrap();

        if let Some(last_number) = last_number {
            if last_number < number {
                larger_count += 1;
            }
        }
        last_number = Some(number);

        if i >= 2 {
            let window_sum = window.into_iter().sum();

            if let Some(last_window_sum) = last_window_sum {
                if last_window_sum < window_sum {
                    larger_window_count += 1;
                }
            }

            last_window_sum = Some(window_sum);
        }
        window[i % 3] = number;
    }

    println!(
        "Number of larger measurements than previous: {}",
        larger_count
    );
    println!(
        "Number of larger three-measurement windows than previous: {}",
        larger_window_count
    );
}
