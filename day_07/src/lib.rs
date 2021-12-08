pub fn least_fuel_position(positions: &mut [i32]) -> i32 {
    positions.sort_unstable();
    let mid = positions[positions.len() / 2];

    positions
        .iter()
        .fold(0, |fuel, position| fuel + (mid - position).abs())
}

fn fuel_cost(p1: i32, p2: i32) -> i32 {
    let distance = (p1 - p2).abs();

    distance * (distance + 1) / 2
}

pub fn least_fuel_inc_position(positions: &[i32]) -> i32 {
    let max_pos = positions.iter().copied().max().unwrap();
    (0..max_pos)
        .map(|align_position| {
            positions.iter().fold(0, |cost, &position| {
                cost + fuel_cost(position, align_position)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_fuel_position() {
        let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(least_fuel_position(&mut positions), 37);
    }

    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(16, 5), 66);
    }

    #[test]
    fn test_least_fuel_inc_position() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(least_fuel_inc_position(&positions), 168);
    }
}
