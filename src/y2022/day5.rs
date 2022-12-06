pub mod day5 {
    use std::{io::{Read, BufReader, BufRead}, collections::VecDeque, num, fs::File};

    pub fn solve_part(filename: &str, scenario: Scenario) {
        let result = rearrange(File::open(filename).unwrap(), scenario);
        println!("{}", result);
    }

    struct CargoStacks {
        stacks: Vec<Vec<char>>
    }

    impl CargoStacks {
        fn new(num_stacks: usize) -> CargoStacks {
            let mut stacks = Vec::new();
            for _  in 0..num_stacks {
                stacks.push(Vec::new());
            }
            CargoStacks { stacks }
        }

        fn push(&mut self, column_number: usize, letter: char) {
            self.stacks.get_mut(column_number).unwrap().push(letter);
        }

        fn pop(&mut self, column_number: usize) -> char {
            self.stacks.get_mut(column_number).unwrap().pop().unwrap()
        }

        fn move_box(&mut self, from_column: usize, to_column: usize) {
            let box_item = self.stacks.get_mut(from_column).unwrap().pop().unwrap();
            self.stacks.get_mut(to_column).unwrap().push(box_item);
        }
    }

    pub enum Scenario {
        One, Two
    }

    fn rearrange<T: Read>(input: T, scenario: Scenario) -> String {
        let reader = BufReader::new(input).lines();

        //We care about two kinds of lines, one that describe the initial box stack configuration
        //and those containing instructions. We'll collect them in two different vectors since
        //they'll be parsed separately
        let mut box_config_stack = Vec::new();
        let mut instructions = Vec::new();

        for line in reader {
            let line = line.unwrap();
            
            //The box configuration will contain at least one [letter] so if the line
            //contains a square bracket we know that's for our box config
            if line.contains("[") {
                box_config_stack.push(format!("{} ", line));
            
            //Instruction lines start with the string "move"
            } else if line.starts_with("move") {
                instructions.push(line);
            }
            //Everything else should be ignored
        }

        //Parse the box config by popping off the box config lines. We want them in reverse
        //order since we can to load them onto our stacks with the bottom first
        let num_columns = box_config_stack.get(0).unwrap().len() / 4;
        let mut stacks = CargoStacks::new(num_columns);
        while let Some(line) = box_config_stack.pop() {
            line.chars()
                .enumerate()
                //Pattern is '[X] ' or '    '
                //repeating every 4 chars. The first is either a space
                //or a square bracket then the box number
                //So after the first (which is index 1), we'll find another
                //letter every 4 so we want to subtract 1 and mod 4 to find the
                //letters
                .filter(|(index, _)| index > &0 && (index - 1) % 4 == 0)
                .for_each(|(index, letter)| {
                    println!("{}", index);
                    let index = (index - 1) / 4;
                    if letter.is_alphabetic() {
                        println!("pushing {} to column {}", letter, index);
                        stacks.push(index, letter);
                    }
                });
        }

        //Now parse the instructions
        for line in instructions {
            //instructions of of the form: 'move <num> from <source> to <dest>'
            //using 1 indexing

            let mut parts = line.split(" ");
            let move_str = parts.next().unwrap();
            let num_boxes = parts.next().unwrap();
            let from_str = parts.next().unwrap();
            let source = parts.next().unwrap();
            let to_str = parts.next().unwrap();
            let dest = parts.next().unwrap();

            if move_str != "move" || from_str != "from" || to_str != "to" {
                panic!("Instructions should be of the form  'move <num> from <source> to <dest>'");
            }

            let num_boxes = num_boxes.parse::<u32>().unwrap();
            let source = source.parse::<usize>().unwrap();
            let dest = dest.parse::<usize>().unwrap();

            match scenario {
                //Moving one at a time so pop the item off and immediately move it to the next
                //column
                Scenario::One => {
                    for _ in 0..num_boxes {
                        //1 indexed so we subtract 1 since the stacks are 0 indexed
                        stacks.move_box(source - 1, dest - 1);
                    }
                },
                //Can move multiple at once and preserve order, so pop them off storing in a temp
                //stack and then pop off that stack onto the destination
                Scenario::Two => {
                    let mut tmp_box_stack = Vec::new();
                    for _ in 0..num_boxes {
                        let box_item = stacks.pop(source - 1);
                        tmp_box_stack.push(box_item);
                    }
                    while let Some(box_item) = tmp_box_stack.pop() {
                        stacks.push(dest - 1, box_item);
                    }
                }
            }
        }

        let mut result = String::new();
        for i in 0..num_columns {
            let top_box = stacks.pop(i);
            result.push(top_box);
        }
        result

    }

    #[test]
    fn test_1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = rearrange(input.as_bytes(), Scenario::One);

        assert_eq!(result,  "CMZ");
    }

    #[test]
    fn test_2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = rearrange(input.as_bytes(), Scenario::Two);

        assert_eq!(result,  "MCD");
    }
}