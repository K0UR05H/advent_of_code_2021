use day_05::{Line, Point};
use std::collections::HashMap;

const INPUT: &str = include_str!("input");

fn main() {
    let lines = INPUT
        .lines()
        .flat_map(|line| line.parse::<Line>())
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .map(|line| line.points())
        .collect::<Vec<Vec<Point>>>();

    let mut overlaps: HashMap<Point, usize> = HashMap::new();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            for point in &lines[i] {
                if lines[j].contains(point) {
                    *overlaps.entry(*point).or_default() += 1;
                }
            }
        }
    }

    println!("{}", overlaps.len());
}
