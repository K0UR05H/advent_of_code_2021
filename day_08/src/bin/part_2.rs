use std::collections::HashMap;

const INPUT: &str = include_str!("input");

fn main() {
    let input = INPUT
        .lines()
        .flat_map(|line| line.split_once(" | "))
        .map(|(patterns, output)| {
            (
                patterns.split_whitespace().collect::<Vec<&str>>(),
                output.split_whitespace().collect::<Vec<&str>>(),
            )
        });

    let mut total = 0;
    for (patterns, output) in input {
        let patterns_map = decode_patterns(&patterns);

        let output: u32 = output
            .iter()
            .map(|digit| sort_segments(digit))
            .map(|digit| patterns_map[&digit])
            .map(|digit| digit.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();

        total += output;
    }

    println!("{}", total);
}

fn sort_segments(segments: &str) -> String {
    let mut segments = segments.chars().collect::<Vec<char>>();
    segments.sort_unstable();
    String::from_iter(segments.into_iter())
}

fn segments_overlap(s1: &str, s2: &str) -> bool {
    s2.chars().all(|c| s1.contains(c))
}

fn decode_patterns(patterns: &[&str]) -> HashMap<String, u8> {
    let mut result: [String; 10] = Default::default();

    for pattern in patterns {
        match pattern.len() {
            2 => result[1] = pattern.to_string(),
            4 => result[4] = pattern.to_string(),
            3 => result[7] = pattern.to_string(),
            7 => result[8] = pattern.to_string(),
            _ => (),
        }
    }

    for pattern in patterns.iter().filter(|p| p.len() == 6) {
        if !segments_overlap(pattern, &result[1]) {
            result[6] = pattern.to_string();
        } else if !segments_overlap(pattern, &result[4]) {
            result[0] = pattern.to_string();
        } else {
            result[9] = pattern.to_string();
        }
    }

    for pattern in patterns.iter().filter(|p| p.len() == 5) {
        if segments_overlap(pattern, &result[1]) {
            result[3] = pattern.to_string();
        } else if segments_overlap(&result[9], pattern) {
            result[5] = pattern.to_string();
        } else {
            result[2] = pattern.to_string();
        }
    }

    let result = result.iter().map(|p| sort_segments(p)).zip(0..);
    HashMap::from_iter(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_segments() {
        assert_eq!(sort_segments("acedgfb"), "abcdefg".to_string());
    }

    #[test]
    fn test_decode_patterns() {
        let patterns = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];

        let mut expected = HashMap::new();
        expected.insert("abcdeg".to_string(), 0);
        expected.insert("ab".to_string(), 1);
        expected.insert("acdfg".to_string(), 2);
        expected.insert("abcdf".to_string(), 3);
        expected.insert("abef".to_string(), 4);
        expected.insert("bcdef".to_string(), 5);
        expected.insert("bcdefg".to_string(), 6);
        expected.insert("abd".to_string(), 7);
        expected.insert("abcdefg".to_string(), 8);
        expected.insert("abcdef".to_string(), 9);

        assert_eq!(decode_patterns(&patterns), expected);
    }
}
