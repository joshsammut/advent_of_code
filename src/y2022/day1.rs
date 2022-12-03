pub mod day1 {
    use std::{fs::{File}, io::{BufReader, BufRead, Read}};


    pub fn solve(filepath: &str) -> u32 {

        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        get_max(reader)
    }

    fn get_max<T: Read>(reader: BufReader<T>) -> u32 {
        let mut max = 0;
        let mut current = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            if line == "" {
                if current > max {
                    max = current;
                }
                current = 0;
            } else {
                let num = line.parse::<u32>().unwrap();
                current += num;
            }
        }

        if current > max {
            max = current;
        }

        println!("max: {}", &max);
        max
    }

    #[test]
    fn should_get_max() {
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
        let max = get_max(BufReader::new(input.as_bytes()));

        assert_eq!(max, 24000)
    }


} 