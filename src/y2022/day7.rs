pub mod day7 {
    use std::{io::{Read, BufReader, BufRead}, collections::HashMap, thread::current, fs::File};

    pub fn part1(filename: &str) {
        let sum = get_small_dir_sizes(File::open(filename).unwrap());
        println!("{}", sum);
    }

    pub fn get_small_dir_sizes<T: Read>(input: T) -> u32 {
        let sizes = get_dir_sizes(input);
        dbg!(sizes.clone());
        sizes.iter()
            .map(|(_, size)| size)
            .filter(|&size| size <= &100000)
            .sum::<u32>()
    }

    pub fn part2(filename: &str) {
        let smallest_to_delete = get_smallest_to_delete(File::open(filename).unwrap());

        println!("{}", smallest_to_delete);
    }

    fn get_smallest_to_delete<T: Read>(input: T) -> u32 {
        let mut sizes = get_dir_sizes(input);

        let total_used = sizes.get("/").unwrap();
        let needed_to_free = 30000000 - (70000000 - total_used);

        let mut sizes = sizes.iter()
            .map(|(_, &size)| size)
            .collect::<Vec<u32>>();

        sizes.sort();

        for size in sizes {
            if size >= needed_to_free {
                return size;
            }
        }
        panic!("Couldn't find a file big enough!");
    }

    enum FileType {
        PlainFile(String, u32),
        Directory(String, Box<Vec<FileType>>)
    }

    fn get_dir_sizes<T: Read>(input: T) -> HashMap<String, u32> {

        let mut dir_stack: Vec<String> = Vec::new();
        let mut dir_sizes: HashMap<String, u32> = HashMap::from([(String::from("/"), 0)]); //running total of directory disk size
        let mut current_dir = String::from("");

        for line in BufReader::new(input).lines() {
            let line = line.unwrap();

            println!("{}", line);

            let parts: Vec<&str> = line.split(" ").collect();

            if parts[0] == "$" {
                match parts[1] {
                    "cd" => {
                        //Move up a directory; pop the stack, update the current dir and add the old directory size to the new
                        //directory's size
                        if parts[2] == ".." {
                            let old_dir = current_dir.clone();
                            let dir_size = dir_sizes.get(&current_dir).unwrap().clone();
                            current_dir = dir_stack.pop().unwrap();
                            println!("moving up {} -> {}, adding {} to {}", old_dir, current_dir, dir_size, current_dir);
                            *dir_sizes.get_mut(&current_dir).unwrap() += dir_size;
                        //Move down a directory, change the current dir
                        } else {
                            let old_dir = current_dir.clone();
                            if current_dir != "" {
                                dir_stack.push(current_dir);
                                current_dir = format!("{}/{}", old_dir, parts[2].to_string());
                            } else {
                                current_dir = "/".to_string();
                            }
                            println!("moving down {} -> {}", old_dir, current_dir);
                        }
                    },
                    "ls" => {
                        //ignore
                    },
                    _ => panic!("unrecognized command")
                }
            } else {
                let identifier = parts[0];
                let file_name = format!("{}/{}", current_dir, parts[1]);

                //New directory, add it with 0 size to the map
                if identifier == "dir" {
                    dir_sizes.insert(file_name.to_string(), 0);
                
                //Regular file, add its size to the current directory's size
                } else if let Ok(file_size) = identifier.parse::<u32>() {
                    println!("adding {} to {}", file_size, current_dir);
                    *dir_sizes.get_mut(&current_dir).unwrap() += file_size;
                } else {
                    panic!("Invalid input");
                }
            }
        }

        while dir_stack.len() > 0 {
            let dir_size = dir_sizes.get(&current_dir).unwrap().clone();
            current_dir = dir_stack.pop().unwrap();
            println!("adding {} to {}", dir_size, current_dir);
            *dir_sizes.get_mut(&current_dir).unwrap() += dir_size;
        }

        dir_sizes
    }

    #[test]
    fn test_1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir d
29116 f
2557 g
62596 h.lst
$ cd d
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let result = get_small_dir_sizes(input.as_bytes());

        assert_eq!(result, 95437);
    }

    #[test]
    fn test_2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir d
29116 f
2557 g
62596 h.lst
$ cd d
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let result = get_smallest_to_delete(input.as_bytes());

        assert_eq!(result, 24933642);
    }
}
