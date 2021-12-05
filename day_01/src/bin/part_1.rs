const INPUT: &str = include_str!("input");

fn main() {
    let (_, inc) =
        INPUT
            .lines()
            .flat_map(|line| line.parse())
            .fold((usize::MAX, 0), |mut state, x| {
                if x > state.0 {
                    state.1 += 1;
                }

                (x, state.1)
            });

    println!("{}", inc);
}
