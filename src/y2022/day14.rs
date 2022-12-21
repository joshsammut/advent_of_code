use std::{io::{BufReader, Read, BufRead}, cmp::{min, max}, collections::HashMap};

pub fn drop_all_sand<T: Read>(input: T) -> u32 {
    let mut terrain = parse_input(input, false);

    while let Some(_) = drop_sand(500, 0, &mut terrain) {}

    println!("sand count: {}", terrain.get_sand_count());
    terrain.get_sand_count()
}

pub fn drop_all_sand_to_floor<T: Read>(input: T) -> u32 {
    let mut terrain = parse_input(input, true);

    while let Some(_) = drop_sand(500, 0, &mut terrain) {}

    println!("sand count: {}", terrain.get_sand_count());
    terrain.get_sand_count()
}

fn drop_sand(x: u32, y: u32, terrain: &mut Terrain) -> Option<()> {
    let mut x_old = x;
    let mut y_old = y;
    loop {
        let next_spot = terrain.find_next_spot(x_old, y_old);


        match next_spot {
            Some((x_new, y_new)) => {
                if x_new == x_old && y_new == y_old {
                    terrain.place_sand(x_new, y_new);
                    break;
                } else {
                    x_old = x_new;
                    y_old = y_new;
                }
            }
            None => return None
        }
    }
    Some(())
}

#[derive(Clone, Debug)]
enum Material {
    Rock,
    Sand
}

#[derive(Debug)]
struct Terrain {
    terrain_map: HashMap<(u32, u32), Material>,
    x_min: u32,
    x_max: u32,
    y_floor: u32,
    use_floor: bool
}

impl Terrain {
    fn new(terrain_map: HashMap<(u32, u32), Material>, use_floor: bool) -> Terrain {
        let x_min = terrain_map.keys()
            .map(|(x, y)| *x)
            .min().unwrap();

        let x_max = terrain_map.keys()
            .map(|(x, y)| *x)
            .max().unwrap();

        let y_max = terrain_map.keys()
            .map(|(x, y)| *y)
            .max().unwrap();

        Terrain{ terrain_map, x_min, x_max, y_floor: y_max + 2, use_floor}
    }

    fn get_sand_count(&self) -> u32 {
        self.terrain_map.values()
            .filter(|m| matches!(m, Material::Sand))
            .count() as u32
    }

    /**
     * Given a grain of sand is in the provided location, find it's next valid spot.
     * It can either go down (y+1) by 1. If that spot is filled then it
     * can move diagonal left by 1 (y+1, x-1). If that is filled then it can move
     * diagonally right by 1 (y+1, x+1). IF that is filled then it must stay where it is.
     * THis method will return the new location without modifying the terrain (you'll have to populate
     * it).
     * 
     * This will return an option. If it returns Some then it'sa valid location (could be the same as the input)
     * If it returns None then there is no valid location to place the sand
     */
    fn find_next_spot(&self, x: u32, y: u32) -> Option<(u32, u32)> {
        //4 options: directly below, diagonal left, diagonal right or where it is
        let options = [(x, y + 1), (x -1, y + 1), (x+1, y+1), (x, y)];

        for option in options {
            let material = self.terrain_map.get(&(option.0, option.1));
            match material {
                //Nothing fills that spot, return those coodinates
                None => {
                    //Check for bounds if we're not using floor, if the next valid space is beyond the extends
                    //then there is no valid spot
                    if !self.use_floor && (option.0 <self.x_min || option.0 > self.x_max) {
                        return None;

                    //If we're using the floor and we've hit the floor, try the next spot
                    } else if self.use_floor && option.1 >= self.y_floor {
                        continue;
                    } else {
                        return Some(option)
                    }
                },

                //Something is there, move on to the next
                Some(_) => continue
            }
        }
        None
    }

    fn place_sand(&mut self, x: u32, y: u32) {
        self.terrain_map.insert((x, y), Material::Sand);
    }
}

fn parse_input<T: Read>(input: T, use_floor: bool) -> Terrain {
    let mut terrain = HashMap::new();
    let mut coordinates = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut rock_line = Vec::new();
        for part in line.split(" -> ") {
            let mut coordinates = part.split(",");
            let x = coordinates.next().unwrap().parse::<u32>().unwrap();
            let y = coordinates.next().unwrap().parse::<u32>().unwrap();
            rock_line.push((x, y));
        }
        coordinates.push(rock_line);
    }

    for line in coordinates {
        for i in 0..line.len() - 1 {
            let (x1, y1) = line.get(i).unwrap().to_owned();
            let (x2, y2) = line.get(i+1).unwrap().to_owned();

            if x2 == x1 {
                //varies in the y direction
                for y in min(y1, y2)..max(y1, y2)+1 {
                    terrain.insert((x2, y), Material::Rock);
                }
            } else if y2 == y1 {
                //varies in the x direction
                for x in min(x1, x2)..max(x1, x2)+1 {
                    terrain.insert((x, y2), Material::Rock);
                }

            } else {
                panic!("diagonal rocks!");
            }
        }
    }
    Terrain::new(terrain, use_floor)
}

#[test]
fn test_1() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9".as_bytes();

    let result = drop_all_sand(input);

    assert_eq!(result, 24);

}

#[test]
fn test_2() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9".as_bytes();

    let result = drop_all_sand_to_floor(input);

    assert_eq!(result, 93);

}