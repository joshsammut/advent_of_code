pub mod day3 {
    use std::{io::{BufReader, Read, BufRead}, fs::File, collections::{HashMap, HashSet}};

    pub fn solve_part1(filepath: &str) {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let total = calculate_rearrange_1(reader);
        println!("{}", total);
    }

    fn calculate_rearrange_1<T: Read>(reader: BufReader<T>) -> u32 {

        let mut total_priority = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let num_items = line.len();

            let mut seen_items = HashSet::new();

            let mut duplicate = None;
            for (index, item) in line.chars().enumerate() {
                //FIrst half, record all the items we saw
                if index < (num_items / 2) {
                    seen_items.insert(item);
                } else {
                    //Now for each item we see, check if we've seen them in rucksack 1
                    match seen_items.take(&item) {
                        Some(item) => {
                            duplicate = Some(item);
                            break;
                        },
                        None => continue
                    }
                }
            }

            total_priority += get_priority(duplicate.unwrap());
        }
        total_priority
    }

    fn get_priority(item: char) -> u32 {
        match item {
            'A'..='Z' => (item as u32) - 38,
            'a'..='z' => (item as u32) - 96,
            _ => panic!("Invalid item")
        }
    }

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = calculate_rearrange_1(BufReader::new(input.as_bytes()));

        assert_eq!(result, 157);
    }

    pub fn solve_part2(filepath: &str) {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let total = calculate_rearrange_2(reader);
        println!("{}", total);
    }


    fn calculate_rearrange_2<T: Read>(reader: BufReader<T>) -> u32 {

        let mut items1 = HashSet::new();
        let mut items2 = HashSet::new();
        let mut items3 = HashSet::new();

        let mut total_priority = 0;
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            match index % 3 {
                //First line in group, populate sest of items in first rucksack
                0 => {
                    //For the first line in the group, wipe out the existing sets
                    items1.clear();
                    items2.clear();
                    items3.clear();
                    for item in line.chars() {
                        items1.insert(item);
                    }
                },

                //Second line in group, populate second set only with items in line 2 and in set 1
                1 => {
                    for item in line.chars() {
                        if let Some(item) = items1.take(&item) {
                            items2.insert(item);
                        }
                    }
                },

                //Last line in group, populate thrid set only with items in set 2 (and thus set 1 too) and in line 3
                2 => {
                    for item in line.chars() {
                        if let Some(item) = items2.take(&item) {
                            items3.insert(item);
                        }
                    }

                    //Now items3 should only have one item
                    if items3.len() != 1 {
                        panic!("wrong number of duplicates, found {}", items3.len())
                    } else {
                        total_priority += get_priority(*items3.iter().next().unwrap())
                    }
                },
                _ => panic!("can't happen")
            }
        }

        total_priority
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = calculate_rearrange_2(BufReader::new(input.as_bytes()));

        assert_eq!(result, 70);
    }
}