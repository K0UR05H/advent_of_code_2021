use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day_1").expect("input file not found!");
    let reader = BufReader::new(file);

    let (_, inc) = reader
        .lines()
        .map(|result| result.unwrap())
        .map(|line| line.parse().unwrap())
        .fold((usize::MAX, 0), |mut state, x| {
            if x > state.0 {
                state.1 += 1;
            }

            (x, state.1)
        });

    println!("{}", inc);
}
