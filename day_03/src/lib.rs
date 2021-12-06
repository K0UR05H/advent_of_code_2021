pub fn count_ones(report: &[&str], position: usize) -> usize {
    report
        .iter()
        .flat_map(|number| number.chars().nth(position))
        .filter(|&digit| digit == '1')
        .count()
}

fn filter_report(report: Vec<&str>, position: usize, bit_criteria: char) -> Vec<&str> {
    report
        .into_iter()
        .filter(|value| value.chars().nth(position).unwrap() == bit_criteria)
        .collect()
}

fn find_oxygen_bit_criteria(input: &[&str], position: usize) -> char {
    let ones = count_ones(input, position);
    let zeros = input.len() - ones;

    if ones >= zeros {
        '1'
    } else {
        '0'
    }
}

fn find_co2_bit_criteria(input: &[&str], position: usize) -> char {
    let ones = count_ones(input, position);
    let zeros = input.len() - ones;

    if zeros <= ones {
        '0'
    } else {
        '1'
    }
}

pub fn find_oxygen_rating(mut report: Vec<&str>) -> u32 {
    let mut position = 0;
    loop {
        let bit_criteria = find_oxygen_bit_criteria(&report, position);
        report = filter_report(report, position, bit_criteria);

        if report.len() == 1 {
            return u32::from_str_radix(report[0], 2).unwrap();
        } else {
            position += 1;
        }
    }
}

pub fn find_co2_rating(mut report: Vec<&str>) -> u32 {
    let mut position = 0;
    loop {
        let bit_criteria = find_co2_bit_criteria(&report, position);
        report = filter_report(report, position, bit_criteria);

        if report.len() == 1 {
            return u32::from_str_radix(report[0], 2).unwrap();
        } else {
            position += 1;
        }
    }
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

    #[test]
    fn test_filter_report() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let expected = vec![
            "11110", "10110", "10111", "10101", "11100", "10000", "11001",
        ];

        assert_eq!(filter_report(report, 0, '1'), expected);
    }

    #[test]
    fn test_oxygen_bit_criteria() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(find_oxygen_bit_criteria(&report, 0), '1');
    }

    #[test]
    fn test_co2_bit_criteria() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(find_co2_bit_criteria(&report, 0), '0');
    }

    #[test]
    fn test_oxygen_rating() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(find_oxygen_rating(report), 23);
    }

    #[test]
    fn test_co2_rating() {
        let report = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(find_co2_rating(report), 10);
    }
}
