use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_1").expect("input file not found!");
    let reader = BufReader::new(file);

    let depths = reader
        .lines()
        .map(|result| result.unwrap())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut inc = 0;
    for i in 3..depths.len() {
        let first_window: usize = depths[i - 3..=i - 1].iter().sum();
        let second_window: usize = depths[i - 2..=i].iter().sum();

        if second_window > first_window {
            inc += 1;
        }
    }

    println!("{}", inc);
}
