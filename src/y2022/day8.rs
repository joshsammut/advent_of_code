use std::{io::{Read, BufReader, BufRead}, str::from_utf8, collections::HashSet, cmp::Ordering, fs::File, num};

pub fn part_1(filename: &str) {
    let num_visible = count_visible_trees(File::open(filename).unwrap());
    println!("{}", num_visible);
}

pub fn part_2(filename: &str) {
    let ideal_score = get_highest_score(File::open(filename).unwrap());
    println!("{}", ideal_score);
}

fn count_visible_trees<T: Read>(input: T) -> u32 {

    let mut visible_trees = HashSet::new();
    let trees = parse_input(input);
    let num_rows = &trees.len();
    let num_columns = &trees.get(0).unwrap().len();

    for row in 0..*num_rows {
        //First go left to right on the row, looking for the tallest yet
        //If a tree is taller than the tallest then it's visible on the left
        //and that's the new tallest
        let mut tallest_left = None;
        for column in 0..*num_columns {
            let size = trees.get(row).unwrap().get(column).unwrap();
            //println!("checking ({},{})", row, column);
            if tallest_left.is_none() || size > tallest_left.unwrap() {
                //println!("next tallest: ({},{})", row_index, column_index);
                visible_trees.insert((row, column));
                tallest_left = Some(size);
            }
        }

        //Nerxt go right to left doing something similar. However once we hit the
        //tallest from the left we can stop since we already know nothing is taller
        //than it
        let mut tallest_right = None;
        for column in (0..*num_columns).rev() {
            let size = trees.get(row).unwrap().get(column).unwrap();
            //println!("jchecking ({},{})", row, column);
            if tallest_right.is_none() || size > tallest_right.unwrap() {
                //println!("next tallest: ({},{})", row_index, column_index);
                visible_trees.insert((row, column));
                tallest_right = Some(size);
            }
            //We've now hit the tallest from the left and can stop (since nothing is strictly
            //taller)
            if size >= tallest_left.unwrap() {
                break;
            }
        }

    }

    //NoVjw we'll go column by column doing the same as above
    for column in 0..*num_columns {
        //Similar to before, go top down finding the next largest tree
        let mut tallest_tree_top = None;
        for row in 0..*num_rows {
            //println!("checking ({},{})", row, column);
            let &size = trees.get(row).unwrap().get(column).unwrap();
            if tallest_tree_top.is_none() || size > tallest_tree_top.unwrap() {
                //println!("next tallest: ({},{})", row, column);
                visible_trees.insert((row, column));
                tallest_tree_top = Some(size);
            }
        }

        //And then go bottom to top
        let mut tallest_tree_bottom = None;
        for row in (0..*num_rows).rev() {
            //println!("checking ({},{})", row, column);
            let &size = trees.get(row).unwrap().get(column).unwrap();
            if tallest_tree_bottom.is_none() || size > tallest_tree_bottom.unwrap() {
                //println!("next tallest: ({},{})", row, column);
                visible_trees.insert((row, column));
                tallest_tree_bottom = Some(size);
            }

            if tallest_tree_bottom.unwrap() >= tallest_tree_top.unwrap() {
                break;
            }
        }
    }

    visible_trees.len() as u32
}

#[derive(Debug)]
struct Score {
    height: u32,
    left: u32,
    right: u32,
    up: u32,
    down: u32
}

impl Score {
    fn new(height: u32) -> Score {
        Score{height, left: 0, right: 0, up: 0, down: 0}
    }

    fn get_score(&self) -> u32 {
        self.left * self.right * self.up * self.down
    }
}

#[derive(Debug)]
struct TreeScores {
    scores: Vec<Vec<Score>>
}

impl<'a> TreeScores {
    fn new(trees: &Vec<Vec<u32>>) -> TreeScores {
        let mut rows = Vec::new();
        for tree_row in trees {
            let mut columns = Vec::new();
            for &height in tree_row {
                columns.push(Score::new(height));
            }
            rows.push(columns);
        }
        TreeScores{ scores: rows}
    }

    fn get(&'a mut self, row: usize, column: usize) -> &'a mut Score {
        self.scores.get_mut(row).unwrap().get_mut(column).unwrap()
    }

    fn get_max_score(&self) -> u32 {
        let mut max = 0;
        for row in &self.scores {
            for score in row {
                let score = score.get_score();
                if score > max {
                    max = score;
                }
            }
        }
        max
    }
}

fn get_highest_score<T: Read>(input: T) -> u32 {
    let trees = parse_input(input);
    let &num_rows = &trees.len();
    let &num_columns = &trees.get(0).unwrap().len();

    let mut viewing_scores = TreeScores::new(&trees);

    for row in 0..num_rows {
        //Stack of the tallest trees. They'll be put in the stack in descending order,
        //so the shortest tree will be at the top of the stack
        let mut tallest_trees: Vec<(usize, u32)> = Vec::new();

        for column in 0..num_columns {
            let current_score = viewing_scores.get(row, column);
            let current_size = current_score.height;

            //We'll keep popping off distances from the stack until we either run out
            //or find a tree taller or equal than the current one
            let mut next_tallest = None;
            while let Some((index, size)) = tallest_trees.pop() {
                if current_size <= size {
                    next_tallest = Some((index, size));
                    break;
                }
            }
            if let Some((index, size)) = next_tallest {
                //The number of trees is the difference between the tallest tree index and the current index
                current_score.left = index.abs_diff(column) as u32;

                //if the tallest is taller than the current one, push it back on. If it's the same
                //we'll leave the duplicate off
                if size > current_size {
                    tallest_trees.push((index, size));
                }
            } else {
                //If we've popped everything off then this is taller than any tree. Therefore the number of trees will be
                //the index
                current_score.left = column as u32;
            }

            tallest_trees.push((column, current_size));
        }

        tallest_trees.clear();
        for column in (0..num_columns).rev() {
            let current_score = viewing_scores.get(row, column);
            let current_size = current_score.height;

            //We'll keep popping off distances from the stack until we either run out
            //or find a tree taller or equal than the current one
            let mut next_tallest = None;
            while let Some((index, size)) = tallest_trees.pop() {
                if current_size <= size {
                    next_tallest = Some((index, size));
                    break;
                }
            }
            if let Some((index, size)) = next_tallest {
                //The number of trees is the difference between the tallest tree index and the current index
                println!("setting distance: ({},{}) => {}", row, column, current_size);
                current_score.right = index.abs_diff(column) as u32;

                //if the tallest is taller than the current one, push it back on. If it's the same
                //we'll leave the duplicate off
                if size > current_size {
                    println!("not the tallest, adding ({},{}) back", row, column);
                    tallest_trees.push((index, size));
                }
            } else {
                //If we've popped everything off then this is taller than any tree. Therefore the number of trees will be
                //the index
                println!("tallest ({},{}) => {}", row, column, current_size);
                current_score.right = (num_columns - 1 - column) as u32;
            }

            tallest_trees.push((column, current_size));
        }
    }

    for column in 0..num_columns {
        //Stack of the tallest trees. They'll be put in the stack in descending order,
        //so the shortest tree will be at the top of the stack
        let mut tallest_trees: Vec<(usize, u32)> = Vec::new();

        for row in 0..num_rows {
            let current_score = viewing_scores.get(row, column);
            let current_size = current_score.height;

            //We'll keep popping off distances from the stack until we either run out
            //or find a tree taller or equal than the current one
            let mut next_tallest = None;
            while let Some((index, size)) = tallest_trees.pop() {
                if current_size <= size {
                    next_tallest = Some((index, size));
                    break;
                }
            }
            if let Some((index, size)) = next_tallest {
                //The number of trees is the difference between the tallest tree index and the current index
                current_score.up = index.abs_diff(row) as u32;

                //if the tallest is taller than the current one, push it back on. If it's the same
                //we'll leave the duplicate off
                if size > current_size {
                    tallest_trees.push((index, size));
                }
            } else {
                //If we've popped everything off then this is taller than any tree. Therefore the number of trees will be
                //the index
                current_score.up = row as u32;
            }

            tallest_trees.push((row, current_size));
        }

        tallest_trees.clear();
        for row in (0..num_rows).rev() {
            let current_score = viewing_scores.get(row, column);
            let current_size = current_score.height;

            //We'll keep popping off distances from the stack until we either run out
            //or find a tree taller or equal than the current one
            let mut next_tallest = None;
            while let Some((index, size)) = tallest_trees.pop() {
                if current_size <= size {
                    next_tallest = Some((index, size));
                    break;
                }
            }
            if let Some((index, size)) = next_tallest {
                //The number of trees is the difference between the tallest tree index and the current index
                println!("setting distance: ({},{}) => {}", row, column, current_size);
                current_score.down = index.abs_diff(row) as u32;

                //if the tallest is taller than the current one, push it back on. If it's the same
                //we'll leave the duplicate off
                if size > current_size {
                    println!("not the tallest, adding ({},{}) back", row, column);
                    tallest_trees.push((index, size));
                }
            } else {
                //If we've popped everything off then this is taller than any tree. Therefore the number of trees will be
                //the index
                println!("tallest ({},{}) => {}", row, column, current_size);
                current_score.down = (num_rows - 1 - row) as u32;
            }

            tallest_trees.push((row, current_size));
        }
    }

    //dbg!(viewing_scores);
    viewing_scores.get_max_score()
}

fn parse_input<T: Read>(input: T) -> Vec<Vec<u32>> {
    let mut trees = Vec::new();
    for line in BufReader::new(input).lines() {
        let line =line.unwrap();
        let mut row = Vec::new();
        for height in line.chars() {
            let height = height.to_digit(10).unwrap();
            row.push(height);
        }
        trees.push(row);
    }
    trees
}

#[test]
fn test_1() {
    let input = "30373
25512
65332
33549
35390";

    let result = count_visible_trees(input.as_bytes());

    assert_eq!(result, 21);
}

#[test]
fn test_2() {
    let input = "30373
25512
65332
33549
35390";

    let result = get_highest_score(input.as_bytes());

    assert_eq!(result, 8);
}



// 123456
// 012345

// 12334356478
// 0121416719(10)