const INPUT: &str = include_str!("input");

fn main() {
    let depths = INPUT
        .lines()
        .flat_map(|line| line.parse())
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
