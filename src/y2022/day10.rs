use std::{io::{Read, BufReader, BufRead}, fs::File, collections::VecDeque, thread::current};

pub fn get_total_signal_strenth<T: Read>(input: T) -> i32 {
    let current_state = State::new(0, 1);
    let mut states = vec![current_state]; //
    let mut cycle_nums = VecDeque::from([20, 60, 100, 140, 180, 220]);
    let mut target_cycle = cycle_nums.pop_front().unwrap();
    let mut signal_values = 0;

    for instruction in parse_input(input) {
        let current_state = states.pop().unwrap();
        let new_state = instruction.execute(&current_state);
        //If the new state is equal or greater than the cycle value we're looking for,
        //Then during the target_cycle the value was the old state's value so calculate
        //the signal strength using that
        println!("cycles finished: {}, xvalue: {}", new_state.tick_number, new_state.register_value);
        if target_cycle <= new_state.tick_number {
            let signal_value = (target_cycle as i32) * current_state.register_value;
            println!("Found cycle {}!!!, value: {}", target_cycle, signal_value);
            signal_values += signal_value;
            let new_target = cycle_nums.pop_front(); 
            match new_target {
                Some(value) => {
                    target_cycle = value;
                },
                None => {
                    break;
                }
            }
        }
        states.push(new_state);
    }
    println!("signal strength: {}", signal_values);
    signal_values
}

pub fn render_crt(filename: &str) {
    let instructions = parse_input(File::open(filename).unwrap());

    let mut current_state = State::new(0, 1);
    let mut screen = [false; 240];

    for instruction in instructions {
        let State { tick_number: old_tick, register_value: old_value } = current_state;
        current_state = instruction.execute(&current_state);
        for tick_number in old_tick..current_state.tick_number {
            let tick_number = (tick_number % 40) as i32;
            let old_value = old_value;
            if tick_number == old_value || tick_number == old_value - 1 || tick_number == old_value + 1 {
                //Value is drawn
                //println!("draw");
                print!("#");
            } else {
                //println!("nope: tick: {}, old_value: {}", tick_number, old_value);
                print!(".");
            }
            if (tick_number + 1) % 40 == 0 {
                print!("\n");
            }
        }
    }
}

#[derive(Debug)]
struct State {
    pub tick_number: u32,
    pub register_value: i32
}

impl State {
    fn new(tick_number: u32, value: i32) -> State  {
        State{ tick_number, register_value: value}
    }

}

enum Instruction {
    Noop,
    Add_X(i32)
}

impl Instruction {
    /**
     * Executes this instruction given the current state value, returning a new state value
     */
    fn execute(&self, current_state: &State) -> State {
        match self {
            Self::Noop => {
                State::new(current_state.tick_number + 1, current_state.register_value)
            },
            Self::Add_X(value) => {
                State::new(current_state.tick_number + 2, current_state.register_value + value)
            }
        }
    }
}

fn parse_input<T: Read>(input: T) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        let instruction = parts.next().unwrap();
        let amount = parts.next();

        match (instruction, amount) {
            ("addx", Some(amount)) => {
                instructions.push(Instruction::Add_X(amount.to_string().parse::<i32>().unwrap()))
            },
            ("noop", None) => {
                instructions.push(Instruction::Noop);
            }
            _ => todo!()
        }
    }
    instructions
}

#[test]
fn test_1() {
    let input = File::open("day10_test.txt").unwrap();

    let result = get_total_signal_strenth(input);

    assert_eq!(result, 13140);
}

#[test]
fn test_2() {
    let result = render_crt("day10_test.txt");
}