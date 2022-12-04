pub mod day2 {
    use std::{io::{BufReader, Read, BufRead}, fs::File, cmp::Ordering};

    #[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
    enum Move {
        Rock = 0,
        Paper = 1,
        Scissors = 2
    }

    impl Move {
        fn from(int: i32) -> Move {
            match int {
                0 => Move::Rock,
                1 => Move::Paper,
                2 => Move::Scissors,
                _ => panic!("invalid int")
            }
        }
    }

    enum Result {
        Win = 6,
        Tie = 3,
        Lose = 0
    }

    pub fn solve_part1(filepath: &str) -> u32 {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let total = calculate_result_part1(reader);
        println!("total score: {}", total);
        total
    }

    pub fn solve_part2(filepath: &str) -> u32 {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let total = calculate_result_part2(reader);
        println!("total score: {}", total);
        total
    }

    fn calculate_result_part1<T: Read>(reader: BufReader<T>) -> u32 {

        let mut total = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_ascii_whitespace();

            let opponent_move = line.next().unwrap();
            let your_move = line.next().unwrap();

            let opponent_move = match opponent_move {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Invalid move")
            };

            let your_move = match your_move {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!("Invalid move")
            };

            //Due to the enum's discriminant we can compare most cases
            //and get the result. Rock <> scissors is the special case
            //and needs to be handled separately
            let result = match (your_move, opponent_move) {
                (Move::Rock, Move::Scissors) => Result::Win,
                (Move::Scissors, Move::Rock) => Result::Lose,
                _ => match your_move.cmp(&opponent_move) {
                    Ordering::Equal => Result::Tie,
                    Ordering::Greater => Result::Win,
                    Ordering::Less => Result::Lose
                }
            };

            let score = (your_move as u32) + 1 + (result as u32);
            total += score;
        }

        total

    }

    fn calculate_result_part2<T: Read>(reader: BufReader<T>) -> u32 {

        let mut total = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_ascii_whitespace();

            let opponent_move = line.next().unwrap();
            let needed_result = line.next().unwrap();

            let opponent_move = match opponent_move {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Invalid move")
            };

            let needed_result = match needed_result {
                "X" => Result::Lose,
                "Y" => Result::Tie,
                "Z" => Result::Win,
                _ => panic!("Invalid move")
            };

            let your_move = match (&needed_result, opponent_move) {
                (Result::Tie, m) => m,
                //For wins we can cast the opponent's move to an int and add one, wrapping
                //around. This will get the needed move to win (since it's defined
                //as rock, paper, scissors)
                (Result::Win, m) => {
                    Move::from(( (m as i32) + 1 ) % 3 )
                },
                //For loses we can cast the opponent's move and subtract  one. If we're negative
                //that means opponent is a rock so we'll need scissors (or 2)
                (Result::Lose, m) => {
                    let mut needed = (m as i32) - 1;
                    if needed < 0 {
                        needed = 2;
                    }
                    Move::from(needed)
                }
            };

            let score = (your_move as u32) + 1 + (needed_result as u32);
            total += score;
        }

        total

    }

    #[test]
    fn it_should_calculate_correct() {
        let input = "A Y
B X
C Z";
        let result = calculate_result_part1(BufReader::new(input.as_bytes()));

        assert_eq!(result, 15);
    }

    #[test]
    fn it_should_calculate_correct_2() {
        let input = "A Y
B X
C Z";
        let result = calculate_result_part2(BufReader::new(input.as_bytes()));

        assert_eq!(result, 12);
    }
}