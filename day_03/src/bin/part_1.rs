const INPUT: &str = include_str!("input");

fn main() {
    let report = INPUT.lines().collect::<Vec<&str>>();
    let reports_len = report.first().unwrap().len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..reports_len {
        let ones = day_03::count_ones(&report, i);

        let most_common_bit = (ones > (report.len() / 2)) as u32;
        let least_common_bit = !most_common_bit & 1;

        gamma |= most_common_bit << (reports_len - 1 - i);
        epsilon |= least_common_bit << (reports_len - 1 - i);
    }

    println!("{}", gamma * epsilon);
}
