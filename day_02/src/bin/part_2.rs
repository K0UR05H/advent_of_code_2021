const INPUT: &str = include_str!("input");

fn main() {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for line in INPUT.lines() {
        let (command, units) = line.split_once(' ').unwrap();
        let units: i32 = units.parse().unwrap();

        match command {
            "forward" => {
                pos += units;
                depth += aim * units;
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => (),
        }
    }

    println!("{}", depth * pos);
}
