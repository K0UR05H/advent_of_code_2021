const INPUT: &str = include_str!("input");

fn main() {
    let (numbers, mut boards) = day_04::read_input(INPUT);

    let mut last_score = 0;
    for number in numbers {
        for board in &mut boards {
            day_04::mark_number(board, number);
        }

        let mut i = 0;
        while i < boards.len() {
            if day_04::is_a_winner(&boards[i]) {
                let board = boards.swap_remove(i);
                last_score = day_04::calculate_score(&board, number);
            } else {
                i += 1;
            }
        }
    }

    println!("{}", last_score);
}
