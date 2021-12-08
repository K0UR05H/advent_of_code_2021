const INPUT: &str = include_str!("input");

fn main() {
    let fish = INPUT
        .split(',')
        .map(|f| f.trim().parse().unwrap())
        .collect::<Vec<u8>>();

    println!("{}", day_06::grow(&fish, 256));
}
