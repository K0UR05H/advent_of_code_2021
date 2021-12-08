pub fn least_fuel_position(positions: &mut [i32]) -> i32 {
    positions.sort_unstable();
    let mid = positions[positions.len() / 2];

    let mut fuel = 0;
    for position in positions {
        fuel += (mid - *position).abs();
    }

    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_fuel_position() {
        let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(least_fuel_position(&mut positions), 37);
    }
}
