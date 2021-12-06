type Board<'a> = Vec<Vec<&'a str>>;
type MarkedBoard<'a> = Vec<Vec<(&'a str, bool)>>;

pub fn read_numbers(numbers: &str) -> Vec<&str> {
    numbers.split(',').collect()
}

pub fn read_board(board: &str) -> Board {
    let mut result = Vec::new();

    for line in board.lines() {
        let row = line.split_whitespace().collect();
        result.push(row);
    }

    result
}

pub fn to_marked_board(board: Board) -> MarkedBoard {
    board
        .into_iter()
        .map(|row| row.into_iter().map(|number| (number, false)).collect())
        .collect()
}

pub fn read_input(input: &str) -> (Vec<&str>, Vec<MarkedBoard>) {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let numbers = read_numbers(input[0]);
    let boards = input
        .iter()
        .skip(1)
        .map(|board| read_board(board))
        .map(to_marked_board)
        .collect();

    (numbers, boards)
}

pub fn mark_number(board: &mut MarkedBoard, number: &str) {
    for row in board {
        for num in row {
            if num.0 == number {
                num.1 = true;
            }
        }
    }
}

pub fn is_a_winner(board: &MarkedBoard) -> bool {
    let columns = board.first().map(|row| row.len()).unwrap_or_default();

    for row in board {
        if row.iter().all(|num| num.1) {
            return true;
        }
    }

    for column in 0..columns {
        if board.iter().all(|row| row[column].1) {
            return true;
        }
    }

    false
}

pub fn calculate_score(board: &MarkedBoard, last_number: &str) -> u32 {
    let mut score = 0;

    for row in board {
        for number in row {
            if !number.1 {
                score += number.0.parse::<u32>().unwrap();
            }
        }
    }

    score * last_number.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_numbers() {
        let numbers = "7,4,9,5,11";
        let expected = vec!["7", "4", "9", "5", "11"];

        assert_eq!(read_numbers(numbers), expected);
    }

    #[test]
    fn test_read_board() {
        let board = "\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19";
        let expected = vec![
            vec!["22", "13", "17", "11", "0"],
            vec!["8", "2", "23", "4", "24"],
            vec!["21", "9", "14", "16", "7"],
            vec!["6", "10", "3", "18", "5"],
            vec!["1", "12", "20", "15", "19"],
        ];

        assert_eq!(read_board(board), expected);
    }

    #[test]
    fn test_to_marked_board() {
        let board = vec![
            vec!["22", "13", "17", "11", "0"],
            vec!["8", "2", "23", "4", "24"],
            vec!["21", "9", "14", "16", "7"],
            vec!["6", "10", "3", "18", "5"],
            vec!["1", "12", "20", "15", "19"],
        ];
        let expected = vec![
            vec![
                ("22", false),
                ("13", false),
                ("17", false),
                ("11", false),
                ("0", false),
            ],
            vec![
                ("8", false),
                ("2", false),
                ("23", false),
                ("4", false),
                ("24", false),
            ],
            vec![
                ("21", false),
                ("9", false),
                ("14", false),
                ("16", false),
                ("7", false),
            ],
            vec![
                ("6", false),
                ("10", false),
                ("3", false),
                ("18", false),
                ("5", false),
            ],
            vec![
                ("1", false),
                ("12", false),
                ("20", false),
                ("15", false),
                ("19", false),
            ],
        ];

        assert_eq!(to_marked_board(board), expected);
    }

    #[test]
    fn test_mark_board() {
        let mut board = vec![
            vec![
                ("22", false),
                ("13", false),
                ("17", false),
                ("11", false),
                ("0", false),
            ],
            vec![
                ("8", false),
                ("2", false),
                ("23", false),
                ("4", false),
                ("24", false),
            ],
            vec![
                ("21", false),
                ("9", false),
                ("14", false),
                ("16", false),
                ("7", false),
            ],
            vec![
                ("6", false),
                ("10", false),
                ("3", false),
                ("18", false),
                ("5", false),
            ],
            vec![
                ("1", false),
                ("12", false),
                ("20", false),
                ("15", false),
                ("19", false),
            ],
        ];

        let expected = vec![
            vec![
                ("22", false),
                ("13", false),
                ("17", false),
                ("11", true),
                ("0", false),
            ],
            vec![
                ("8", false),
                ("2", false),
                ("23", false),
                ("4", true),
                ("24", false),
            ],
            vec![
                ("21", false),
                ("9", true),
                ("14", false),
                ("16", false),
                ("7", true),
            ],
            vec![
                ("6", false),
                ("10", false),
                ("3", false),
                ("18", false),
                ("5", true),
            ],
            vec![
                ("1", false),
                ("12", false),
                ("20", false),
                ("15", false),
                ("19", false),
            ],
        ];

        mark_number(&mut board, "7");
        mark_number(&mut board, "4");
        mark_number(&mut board, "9");
        mark_number(&mut board, "5");
        mark_number(&mut board, "11");

        assert_eq!(board, expected);
    }

    #[test]
    fn test_winner_row() {
        let numbers = "7,4,9,5,11,17,23,2,0,14,21";
        let board = "\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7";
        let board = read_board(board);
        let mut board = to_marked_board(board);

        for number in read_numbers(numbers) {
            mark_number(&mut board, number);
            assert_eq!(is_a_winner(&board), false);
        }
        mark_number(&mut board, "24");
        assert_eq!(is_a_winner(&board), true);
    }

    #[test]
    fn test_winner_column() {
        let numbers = "14,17,10,26,18,22";
        let board = "\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7";
        let board = read_board(board);
        let mut board = to_marked_board(board);

        for number in read_numbers(numbers) {
            mark_number(&mut board, number);
            assert_eq!(is_a_winner(&board), false);
        }
        mark_number(&mut board, "2");
        assert_eq!(is_a_winner(&board), true);
    }

    #[test]
    fn test_calculate_score() {
        let numbers = "7,4,9,5,11,17,23,2,0,14,21,24";
        let board = "\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7";
        let board = read_board(board);
        let mut board = to_marked_board(board);

        for number in read_numbers(numbers) {
            mark_number(&mut board, number);
        }

        assert_eq!(calculate_score(&board, "24"), 4512);
    }
}
