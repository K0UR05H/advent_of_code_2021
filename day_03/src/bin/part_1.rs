const INPUT: &str = include_str!("input");

fn main() {
    let numbers = INPUT.lines().collect::<Vec<&str>>();
    let nums_len = numbers.first().unwrap().len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..nums_len {
        let ones = numbers
            .iter()
            .flat_map(|number| number.chars().nth(i))
            .filter(|&digit| digit == '1')
            .count();

        let most_common_bit = (ones > (numbers.len() / 2)) as u32;
        let least_common_bit = !most_common_bit & 1;

        gamma |= most_common_bit << (nums_len - 1 - i);
        epsilon |= least_common_bit << (nums_len - 1 - i);
    }

    println!("{}", gamma * epsilon);
}
