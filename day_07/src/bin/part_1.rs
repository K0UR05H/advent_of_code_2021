const INPUT: &str = include_str!("input");

fn main() {
    let mut positions = INPUT
        .split(',')
        .flat_map(|s| s.trim().parse())
        .collect::<Vec<i32>>();

    println!("{}", day_07::least_fuel_position(&mut positions));
}
