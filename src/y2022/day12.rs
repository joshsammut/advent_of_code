use std::{io::{Read, BufReader, BufRead}, collections::VecDeque, fmt::{Display, Write}};

pub fn find_shortest_from_start<T: Read>(input: T) -> u16 {
    let mut trees = parse_input(input);

    let start = trees.find_start().unwrap();

    find_shortest_route(&mut trees, start.0, start.1).unwrap()
}

pub fn find_any_shortest<T: Read>(input: T) -> u16 {
    let mut trees = parse_input(input);

    let potential_starts = trees.find_height(0);
    let mut min = None;

    for (x_start, y_start) in potential_starts {
        trees.reset();
        let path = find_shortest_route(&mut trees, x_start, y_start);
        match (min, path) {
            (Some(old_min), Some(path)) if path < old_min => {
                min = Some(path);
            },
            (None, path) => {
                min = path;
            },
            _ => { continue; }
        };
    }
    println!("min: {}", min.unwrap());
    min.unwrap()
}

fn find_shortest_route(trees: &mut Trees, x_start: usize, y_start: usize) -> Option<u16> {
    let start = (x_start, y_start);
    trees.get_tree_mut(x_start, y_start).distance_to_start = Some(0);
    let mut nodes_to_check = VecDeque::from([start]);

    while let Some((x, y)) = nodes_to_check.pop_front() {

        let current_dist = {
            trees.get_tree(x, y).distance_to_start
        };

        //get possible neighbour nodes
        let neighbours = trees.find_neighbours(x, y);

        //for each check if current node's distance + 1 is less than neighbour's current distance
        //if so update its distance and add to the queue
        for neighbour in neighbours {
            let neighbour_tree = trees.get_tree_mut(neighbour.0, neighbour.1);
            match (neighbour_tree.distance_to_start, current_dist) {
                (None, Some(current_dist)) => {
                    neighbour_tree.distance_to_start = Some(current_dist + 1);
                    nodes_to_check.push_back(neighbour);
                },
                (Some(neighbour_dist), Some(current_dist))
                    if neighbour_dist > current_dist + 1 =>  {
                    neighbour_tree.distance_to_start = Some(current_dist + 1);
                    nodes_to_check.push_back(neighbour);
                },
                _ => {}
            }
        }
    }

    let end = trees.find_end().unwrap();
    let end = trees.get_tree(end.0, end.1);

    match end.distance_to_start {
        Some(num) => {
            println!("{}", num);
        },
        None => {}
    }
    end.distance_to_start

}

fn parse_input<T: Read>(input: T) -> Trees {
    let trees = BufReader::new(input).lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars()
            .map(|tree| {
                match tree {
                    'S' => TreeNode::new(0, NodeType::Start),
                    'E' => TreeNode::new(25, NodeType::End),
                    other => TreeNode::new(other as u16 - 97, NodeType::Other)
                }
            })
            .collect::<Vec<TreeNode>>()
         )
         .collect::<Vec<Vec<TreeNode>>>();

    Trees::new(trees)
}

#[derive(Debug)]
struct TreeNode {
    height: u16,
    distance_to_start: Option<u16>,
    tree_type: NodeType
}

impl TreeNode {
    fn new(height: u16, tree_type: NodeType) -> TreeNode {
        TreeNode{ height, tree_type, distance_to_start: None}
    }
}

#[derive(Debug)]
struct Trees {
    trees: Vec<Vec<TreeNode>>
}

impl Trees {
    fn new(trees: Vec<Vec<TreeNode>>) -> Trees {
        Trees{trees}
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for (i, line) in self.trees.iter().enumerate() {
            for (j, tree) in line.iter().enumerate() {
                match tree.tree_type {
                    NodeType::Start => {
                        return Some((i, j))
                    },
                    _ => continue
                }
            }
        }
        None
    }

    fn find_end(&self) -> Option<(usize, usize)> {
        for (i, line) in self.trees.iter().enumerate() {
            for (j, tree) in line.iter().enumerate() {
                match tree.tree_type {
                    NodeType::End => {
                        return Some((i, j))
                    },
                    _ => continue
                }
            }
        }
        None
    }

    fn find_height(&self, height: u16) -> Vec<(usize, usize)> {
        let mut trees = Vec::new();
        for (i, line) in self.trees.iter().enumerate() {
            for (j, tree) in line.iter().enumerate() {
                if tree.height == height {
                    trees.push((i, j));
                }
            }
        }
        trees
    }

    fn find_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let current = self.get_tree(x, y);
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.trees.len() - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.trees.get(0).unwrap().len() - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours.iter()
            .filter(|(x, y)| {
                let neighbour = self.get_tree(*x, *y);
                neighbour.height <= current.height + 1
            })
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_tree(&self, x: usize, y: usize) -> &TreeNode {
       self.trees.get(x).unwrap().get(y).unwrap()
    }

    fn get_tree_mut(&mut self, x: usize, y: usize) -> &mut TreeNode {
       self.trees.get_mut(x).unwrap().get_mut(y).unwrap()
    }

    fn reset(&mut self) {
        for line in &mut self.trees {
            for tree in line {
                tree.distance_to_start = None;
            }
        }
    }
}


#[derive(Debug)]
enum NodeType {
    Start,
    End,
    Other
}

#[test]
fn test_1() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let result = find_shortest_from_start(input.as_bytes());

    assert_eq!(result, 31);
}

#[test]
fn test_2() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let result = find_any_shortest(input.as_bytes());

    assert_eq!(result, 29);
}