pub fn count_ones(report: &[&str], position: usize) -> usize {
    report
        .iter()
        .flat_map(|number| number.chars().nth(position))
        .filter(|&digit| digit == '1')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ones() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(count_ones(&report, 0), 7);
    }
}
