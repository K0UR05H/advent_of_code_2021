pub fn grow(fish: &[u8], days: u32) -> usize {
    let mut fish_age = [0; 9];

    for &f in fish {
        fish_age[f as usize] += 1;
    }

    for _ in 0..days {
        fish_age[7] += fish_age[0];
        fish_age.rotate_left(1);
    }

    fish_age.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grow() {
        let fish = vec![3, 4, 3, 1, 2];
        assert_eq!(grow(&fish, 80), 5934);
    }
}
