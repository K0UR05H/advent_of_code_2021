use day_05::{Line, Point};
use std::collections::HashMap;

const INPUT: &str = include_str!("input");

fn main() {
    let lines = INPUT
        .lines()
        .flat_map(|line| line.parse::<Line>())
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .collect::<Vec<Line>>();

    let mut overlaps: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        for point in line.points() {
            *overlaps.entry(point).or_default() += 1;
        }
    }

    let overlaps = overlaps.values().filter(|&value| *value > 1).count();
    println!("{}", overlaps);
}
