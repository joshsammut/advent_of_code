use std::{collections::{VecDeque, HashMap}, io::{Read, BufReader, BufRead, Lines}};

use num_bigint::BigUint;
use regex::Regex;

pub fn find_busy_monkeys<T: Read>(input: T, num_rounds: u32, worry_decrease_rate: u32) -> u32 {
    let mut monkeys = parse_input(input);
    let num_monkeys = monkeys.len();
    for _ in 0..num_rounds {
        for i in 0..num_monkeys {
            let monkey = monkeys.get_mut(i).unwrap();
            let mut to_throw = Vec::new();
            //println!("Monkey {}", monkey.num);
            while let Some(next_item) = monkey.inspect() {
                //println!("worry level is now {}", next_item);
                let next_item = next_item / (worry_decrease_rate as u64);
                //println!("worry level dived by 3 to {}", next_item);
                let target = monkey.get_throw_target(&next_item);
                //println!("thrown to {}", target);
                to_throw.push((target, next_item));
            }
            for (target, next_item) in to_throw {
                monkeys.get_mut(target as usize).unwrap().catch(next_item);
            }
        }
        //dbg!(&monkeys);
    }

    monkeys.sort_by(|a, b| a.num_inspected.cmp(&b.num_inspected));

    let top = monkeys.get(monkeys.len() - 1).unwrap();
    let other = monkeys.get(monkeys.len() - 2).unwrap();

    let monkey_business = top.num_inspected * other.num_inspected;
    println!("{}", monkey_business);
    monkey_business
}

#[derive(Debug)]
struct Monkey {
    num: u32,
    items: VecDeque<BigUint>,
    num_inspected: u32,
    operation: Operation,
    test: InspectionTest
}

#[derive(Debug)]
enum Operation {
    Add(u32), Multiply(u32), Exponent(u32)
}

impl Operation {
    /**
     * Applies this operation to the given item
     */
    fn apply(&self, item:  BigUint) -> BigUint {
        match self {
            Self::Add(num) => item + (*num as u64),
            Self::Multiply(num) => item * (*num as u64),
            Self::Exponent(num) => item.pow(*num)
        }
    }
}

#[derive(Debug)]
struct InspectionTest {
    divisor: u32,
    ifTrue: u32,
    ifFalse: u32
}

impl InspectionTest {
    fn new(divisor: u32, ifTrue: u32, ifFalse: u32) -> InspectionTest {
        InspectionTest { divisor, ifTrue, ifFalse }
    }

    /**
     * Given an item, return the monkye number to throw to
     */
    fn get_throw_target(&self, item: &BigUint) -> u32 {
        if item % (self.divisor as u64) == BigUint::from(0u32) {
            self.ifTrue
        } else {
            self.ifFalse
        }
    }
}

impl Monkey {
    fn new(num: u32, initialItems: Vec<BigUint>, operation: Operation, test: InspectionTest) -> Monkey {
        Monkey {
            num,
            items: VecDeque::from(initialItems),
            num_inspected: 0,
            operation,
            test
        }

    }

    /** 
     * Takes the next available item, spects it and returns
     * the new worry value after the inspection. THe item will
     * be removed from the list of items
    */
    fn inspect(&mut self) -> Option<BigUint> {
        let next_item = self.items.pop_front();
        match next_item {
            Some(next_item) => {
                //println!("Monkey inspects an item with worry level {}", next_item);
                self.num_inspected += 1;
                Some(self.operation.apply(next_item))
            },
            None => None
        }
    }

    fn get_throw_target(&self, item: &BigUint) -> u32 {
        self.test.get_throw_target(item)
    }
    
    fn catch(&mut self, item: BigUint) {
        self.items.push_back(item);
    }
}

fn parse_input<T: Read>(input: T) -> Vec<Monkey> {
    let mut lines = BufReader::new(input).lines();

    let mut monkeys = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let line = line.trim();
        match (line) {
            line if line.starts_with("Monkey") => {
                monkeys.push(parse_next_monkey(&line, &mut lines));
            },
            "" => continue,
            _ => panic!("unexpected input")
        }
    }
    monkeys
}

/**
 * Given a line iterator and the line identifiying a monkey, parses all the necessary
 * information from the iterator, advancing it to the next monkey
 */
fn parse_next_monkey<T: Read>(identifier_line: &str, lines: &mut Lines<BufReader<T>>) -> Monkey {
    //First parse the number from the string
    let monkey_regex = Regex::new(r"Monkey (\d+):").unwrap();
    let cap = monkey_regex.captures_iter(&identifier_line).next().unwrap();
    let num = &cap[1].parse::<u32>().unwrap();

    let items_regex = Regex::new(r"Starting items: (.*)").unwrap();
    let operation_regex = Regex::new(r"Operation: new = old ([\*+]) (.*)").unwrap();
    let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let if_true_regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let if_false_regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    let starting_items = lines.next().unwrap().unwrap();
    let starting_items = items_regex.captures_iter(&starting_items)
        .next().unwrap();
    let starting_items = &starting_items[1];
    let starting_items = starting_items.split(",")
        .map(|i| i.trim())
        .map(|i| i.parse::<BigUint>().unwrap())
        .collect::<Vec<BigUint>>();

    let operation = lines.next().unwrap().unwrap();
    let cap = operation_regex.captures_iter(&operation).next().unwrap();
    let operator = &cap[1];
    let operand = &cap[2];
    let operation = match (operator, operand) {
        ("*", "old") => Operation::Exponent(2),
        ("*", num) if num.parse::<u32>().is_ok() => Operation::Multiply(num.parse::<u32>().unwrap()),
        ("+", num) if num.parse::<u32>().is_ok() => Operation::Add(num.parse::<u32>().unwrap()),
        _ => panic!("invalid operation")
    };

    let test = lines.next().unwrap().unwrap();
    let cap = test_regex.captures_iter(&test).next().unwrap();
    let test = &cap[1].parse::<u32>().unwrap();

    let if_true = lines.next().unwrap().unwrap();
    let cap = if_true_regex.captures_iter(&if_true).next().unwrap();
    let if_true = &cap[1].parse::<u32>().unwrap();

    let if_false = lines.next().unwrap().unwrap();
    let cap = if_false_regex.captures_iter(&if_false).next().unwrap();
    let if_false = &cap[1].parse::<u32>().unwrap();

    let inspection_test = InspectionTest::new(*test, *if_true, *if_false);

    Monkey::new(*num, starting_items, operation, inspection_test)
}

#[test]
fn test_1() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let result = find_busy_monkeys(input.as_bytes(), 20, 3);

    assert_eq!(result, 10605);
}

#[test]
fn test_2() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let result = find_busy_monkeys(input.as_bytes(), 10000, 1);

    assert_eq!(result, 2713310158);
}