const INPUT: &str = include_str!("input");

fn main() {
    let unique_digits: usize = INPUT
        .lines()
        .flat_map(|line| line.split_once(" | "))
        .map(|(_, output)| output.split_whitespace().collect::<Vec<&str>>())
        .map(|digits| count_unique_digits(&digits))
        .sum();

    println!("{}", unique_digits);
}

fn count_unique_digits(digits: &[&str]) -> usize {
    const UNIQUE_DIGITS_LEN: [usize; 4] = [2, 3, 4, 7];

    digits
        .iter()
        .filter(|digit| UNIQUE_DIGITS_LEN.contains(&digit.len()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_digits() {
        let digits = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];
        assert_eq!(count_unique_digits(&digits), 2);

        let digits = vec!["fcgedb", "cgb", "dgebacf", "gc"];
        assert_eq!(count_unique_digits(&digits), 3);
    }
}
