pub mod day1 {
    use std::{fs::{File}, io::{BufReader, BufRead, Read}, collections::btree_map::IterMut};


    pub fn solve_part1(filepath: &str) -> u32 {

        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        get_max_part_1(reader)
    }

    pub fn solve_part2(filepath: &str) -> u32 {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        get_max_part_2(reader)
    }



    fn get_max_part_1<T: Read>(reader: BufReader<T>) -> u32 {
        let mut max = 0;
        let mut current = 0;
        let comparse_and_swap = |current: u32, max:u32| {
            let mut new_max = max; 
            if current > max {
                new_max = current;
            }
            new_max
        };
        for line in reader.lines() {
            let line = line.unwrap();
            if line == "" {
                max = comparse_and_swap(current, max);
                current = 0;
            } else {
                let num = line.parse::<u32>().unwrap();
                current += num;
            }
        }

        max = comparse_and_swap(current, max);

        println!("max: {}", &max);
        max
    }

    fn get_max_part_2<T: Read>(reader: BufReader<T>) -> u32 {
        //Top 3 calories, sorted in ascending order
        let mut max = [0,0,0];
        let mut current = 0;
        let compare_and_swap = |current: u32, max: &mut [u32; 3]| {
                if current > max[0] {
                    max[0] = current;
                    max.sort();
                }
        };
        for line in reader.lines() {
            let line = line.unwrap();
            if line == "" {
                //Since the list is sorted in ascending order,
                //we can just look at the smallest one, swap it
                //out and then sort. Sorting a nearly sorted list
                //should be fast since it likely uses quick sort
                //when the input is small enough
                compare_and_swap(current, &mut max);
                current = 0;
            } else {
                let num = line.parse::<u32>().unwrap();
                current += num;
            }
        }

        //Don't forget the last line!
        compare_and_swap(current, &mut max);

        let sum: u32 = max.iter().sum();

        println!("sum: {}", &sum);
        sum
    }

    #[test]
    fn should_get_max_part_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let max = get_max_part_1(BufReader::new(input.as_bytes()));

        assert_eq!(max, 24000)
    }

    #[test]
    fn should_get_max_part_2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let max = get_max_part_2(BufReader::new(input.as_bytes()));

        assert_eq!(max, 45000)
    }


} 