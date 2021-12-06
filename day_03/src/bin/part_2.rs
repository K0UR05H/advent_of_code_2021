const INPUT: &str = include_str!("input");

fn main() {
    let report = INPUT.lines().collect::<Vec<&str>>();

    let oxygen_rating = day_03::find_oxygen_rating(report.clone());
    let co2_rating = day_03::find_co2_rating(report);

    println!("{}", oxygen_rating * co2_rating);
}
