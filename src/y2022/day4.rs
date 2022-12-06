pub mod day4 {
    use std::{io::{Read, BufReader, BufRead}, fs::File, fmt::Display};

    pub fn solve_part1(filepath: &str) {
        let file = File::open(filepath).unwrap();

        let total = calculate_overlaps_1(file);

        println!("{}", total);
    }

    fn calculate_overlaps_1<T: Read>(input: T) -> u32 {
        let reader = BufReader::new(input);

        let mut num_contains = 0;
        for line in reader.lines() {
            let assignments = parse_line(&line.unwrap());

            if Assignment::either_contains(&assignments[0], &assignments[1]) {
                num_contains += 1;
            }
        }

        num_contains
    }


    #[test]
    fn part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let result = calculate_overlaps_1(input.as_bytes());

        assert_eq!(result, 2);
    }

    pub fn solve_part2(filepath: &str) {
        let file = File::open(filepath).unwrap();

        let total = calculate_overlaps_2(file);

        println!("Day4 part2: {}", total);
    }

    fn calculate_overlaps_2<T: Read>(input: T) -> u32 {
        let reader = BufReader::new(input);

        let mut num_contains = 0;
        for line in reader.lines() {
            let assignments = parse_line(&line.unwrap());

            if Assignment::overlaps(&assignments[0], &assignments[1]) {
                num_contains += 1;
            }
        }

        num_contains
    }

    #[test]
    fn test2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let result = calculate_overlaps_2(input.as_bytes());

        assert_eq!(result, 4);
    }

    struct Assignment {
        start: u32,
        end: u32
    }

    impl Assignment {
        fn new(start: u32, end: u32) -> Assignment {
            Assignment { start, end }
        }

        /**
         * Returns whether this assignments fully contains the provided assignment
         */
        fn contains(&self, other: &Self) -> bool {
            self.start <= other.start && self.end >= other.end
        }

        /**
         * Returns whether either pair fully contains the other, ie A contains B or B contains A
         */
        fn either_contains(first: &Self, second: &Self) -> bool {
            first.contains(second) || second.contains(first)
        }

        fn overlaps(first: &Self, second: &Self) -> bool {
            first.start <= second.start && first.end >= second.start ||
                first.start > second.start && second.end >= first.start
        }

    }

    impl Display for Assignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
    }

    fn parse_line(line: &str) -> [Assignment; 2] {
        let mut parts = line.split(",");
        let first = parts.next().unwrap();
        let first = parse_assignment(first);
        let second = parts.next().unwrap();
        let second = parse_assignment(second);

        [first, second]
    }

    /**
     * Given <num>-<num>, return an assignment
     * IE 3-6 returns Assignment(3, 6)
     */
    fn parse_assignment(assignment: &str) -> Assignment {
        let mut parts = assignment.split("-");

        let start = parts.next().unwrap().parse::<u32>().unwrap();
        let end = parts.next().unwrap().parse::<u32>().unwrap();

        Assignment::new(start, end)
    }


}