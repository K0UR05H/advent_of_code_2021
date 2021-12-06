const INPUT: &str = include_str!("input");

fn main() {
    let (numbers, mut boards) = day_04::read_input(INPUT);

    for number in numbers {
        for board in &mut boards {
            day_04::mark_number(board, number);
            if day_04::is_a_winner(board) {
                println!("{}", day_04::calculate_score(board, number));
                return;
            }
        }
    }
}
