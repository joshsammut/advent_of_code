use std::{io::{Read, BufReader, BufRead}, collections::HashSet, fs::File};

pub fn part_1(input: &str) {
    let num_movements = get_tail_movements(File::open(input).unwrap(), 2);
    println!("{}", num_movements);
}

pub fn part_2(input: &str) {
    let num_movements = get_tail_movements(File::open(input).unwrap(), 10);
    println!("{}", num_movements);
}

fn get_tail_movements<T: Read>(input: T, num_knots: u32) -> u32 {

    let mut tracker = RopeTracker::new(num_knots);
    parse_input(input).iter().for_each(|movement| {
        tracker.move_head(movement);
    });
    tracker.get_num_tail_movements()
}

fn parse_input<T: Read>(input: T) -> Vec<Movement> {

    let mut movements = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let num_movements = parts.next().unwrap().parse::<u32>().unwrap();

        movements.push(Movement::new(Direction::from(direction), num_movements))
    }
    movements
}

struct Movement {
    direction: Direction,
    num_movements: u32
}

impl Movement {
    fn new(direction: Direction, num_movements: u32) -> Movement {
        Movement { direction, num_movements }
    }
}

#[derive(Debug)]
enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    fn from(direction: &str) -> Direction {
        match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction")
        }
    }
}

struct RopeTracker {
    //Position, up is positive, down negative, left negative, right positive
    head_position: (i32, i32),
    tail_position: (i32, i32),
    knots: Vec<(i32, i32)>,
    tail_movements: HashSet<(i32, i32)>
}

impl RopeTracker {
    fn new(num_knots: u32) -> RopeTracker {
        let mut knots = Vec::new();
        for _ in 0..num_knots {
            knots.push((0, 0));
        }
        RopeTracker { 
            head_position: (0, 0),
            tail_position: (0, 0),
            tail_movements: HashSet::from([(0, 0)]),
            knots
        }
    }

    fn get_num_tail_movements(&self) -> u32 {
        self.tail_movements.len() as u32
    }

    fn move_head(&mut self, movement: &Movement) {
        let Movement{direction, num_movements} = movement;

        for _ in 0..*num_movements {
            //Knots are in order so i=0 is the real head and i=len-1 is the real tail
            //However our movement can only handle  knots that are adjacent so take the first
            //as the head and the next ax the tail and move it. Then take that tail as the head
            //and the next to it as the tail
            for i in 0..self.knots.len() - 1 {
                let (head, tail) = unsafe {
                    let mut head = self.knots.get_mut(i).unwrap() as *mut _;
                    let mut tail = self.knots.get_mut(i + 1).unwrap() as *mut _;
                    (&mut *head, &mut *tail)
                };

                //Only for the first do we move the head based on the direction
                //All the other heads will be moved by the previous tail movement
                if i == 0 {
                    let new_head = move_head_one(direction, head);
                    head.0 = new_head.0;
                    head.1 = new_head.1;
                }
                let new_tail = tail_follows(head, tail);
                tail.0 = new_tail.0;
                tail.1 = new_tail.1;

                //dbg!(&self.knots);
            }
            //[self.head_position, self.tail_position] = self.move_head_one(direction, self.head_position, self.tail_position);

            //dbg!(self.knots.get(self.knots.len() - 1).unwrap());
            //Finally update the cache with the true tail's position
            self.tail_movements.insert(*self.knots.get(self.knots.len() - 1).unwrap());
            //println!("{:?}", self.knots);
        }
        //println!("{:?}{} {:?}", movement.direction, movement.num_movements, self.knots);
    }


}

/**
 * Moves the head the given direction
 */
fn move_head_one(direction: &Direction, head: &(i32, i32)) -> (i32, i32) {
    let mut new_head = head.clone();
    //Move head
    match direction {
        Direction::Left => {
            new_head = (head.0 - 1, head.1)
        },
        Direction::Right => {
            new_head = (head.0 + 1, head.1)
        },
        Direction::Up => {
            new_head = (head.0, head.1 + 1 )
        },
        Direction::Down => {
            new_head = (head.0, head.1 - 1)
        },
    }
    new_head
}

/**
 * Given the head (which has just moved), return the new position of the tail
 */
fn tail_follows(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let mut new_tail = tail.clone();
    //Now figure out where tail needs to go
    let (x_diff, y_diff) = find_displacement(head, &tail);

    let mag_diff = x_diff.abs() + y_diff.abs();

    if mag_diff <= 1 || x_diff.abs() == 1 && y_diff.abs() == 1 {
        //We're still touching if the sum of the differences in both directions is 1 or 0
        //or it's exactly 1  in both directions

        //do nothing
    } else if x_diff == 0 {
        //Only move by 1 in y direction
        let amount = if y_diff > 0 { 1 } else {-1 };
        new_tail = (tail.0, tail.1 + amount)
    } else if y_diff == 0 {
        //Only move by 1 in x direction
        let amount = if x_diff > 0 { 1 } else {-1 };
        new_tail = (tail.0 + amount, tail.1)
    } else {
        //Move by 1 in both the x and y direction
        let x_amount = if x_diff > 0 { 1 } else {-1 };
        let y_amount = if y_diff > 0 { 1 } else {-1 };
        new_tail = (tail.0 + x_amount, tail.1 + y_amount);
    }

    // //First if the x direction varies by 2, move x. We'll move y based on its diff
    // //Either we're in line and this diff is 0 or we're on an angle and we need to 
    // //bump up too
    // if x_diff > 1 {
    //     new_tail = (tail.0 + 1, tail.1 + y_diff);
    // } else if x_diff < -1 {
    //     new_tail = (tail.0 - 1, tail.1 + y_diff);
    // //Then for y it's similar to x but flipped
    // } else if y_diff > 1 {
    //     new_tail = (tail.0 + x_diff, tail.1 + 1);
    // } else if y_diff < -1 {
    //     new_tail = (tail.0 + x_diff, tail.1 - 1);
    // }
    new_tail

    ////dbg!(head, tail);
}


/**
 * Finds the displacement between head and tail. Displacement is the
 * difference between positions in both directions
 */
fn find_displacement(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    (head.0 - tail.0, head.1 - tail.1)
}

#[test]
fn test_1() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let result = get_tail_movements(input.as_bytes(), 2);

    assert_eq!(result, 13);
}

#[test]
fn test_2() {
    // let input = "R 4";
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let result = get_tail_movements(input.as_bytes(), 10);

    assert_eq!(result, 1);
}

#[test]
fn test_3() {
    // let input = "R 4";
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let result = get_tail_movements(input.as_bytes(), 10);

    assert_eq!(result, 36);
}