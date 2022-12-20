use std::{io::{Read, BufReader, BufRead}, str::Chars, cmp::Ordering};

pub fn find_score_correctly_ordered<T: Read>(input: T) -> usize {
    let ordered = find_correct_order(input);
    //dbg!(&ordered);
    let sum = ordered.iter().sum();
    println!("{}", sum);
    sum
}

pub fn get_decoder_key<T: Read>(input: T) -> usize {
    let packets = parse_input(input);
    let mut packets = packets.iter()
        .flat_map(|(p1, p2)| [p1, p2])
        .collect::<Vec<&Packet>>();

    //add [[2]] and [[6]] markers
    let inner = Packet::List(vec![Packet::Num(2)]);
    let start_divider = Packet::List(vec![inner]);
    packets.push(&start_divider);
    let inner = Packet::List(vec![Packet::Num(6)]);
    let end_divider = Packet::List(vec![inner]);
    packets.push(&end_divider);

    packets.sort();

    let mut decoder_key = 1;
    for (index, &packet) in packets.iter().enumerate() {
        if packet.eq(&start_divider) || packet.eq(&end_divider) {
            decoder_key *= index + 1;
        }
    }
    println!("Decoder key: {}", decoder_key);
    decoder_key
}

fn find_correct_order<T: Read>(input: T) -> Vec<usize> {
    let packets = parse_input(input);
    let mut in_order = Vec::new();
    //dbg!(&packets);

    for (index, (packet1, packet2)) in packets.iter().enumerate() {
        println!("trying pair {}", index + 1);
        let order = packet1.cmp(packet2);
        match order {
            Ordering::Less => {
                println!("ordered");
                in_order.push(index + 1); //1 indexed
            },
            Ordering::Greater => {
                println!("unordered");
            },
            Ordering::Equal => {
                panic!("list are the same!");
            }
        }
    }

    in_order
}

fn parse_input<T: Read>(input: T) -> Vec<(Packet, Packet)> {
    let mut packet_pairs = Vec::new();
    let mut current_pair: Vec<Packet> = Vec::new();

    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let line = line.trim();

        match line {
            "" => {
                if current_pair.len() != 2 {
                    dbg!(current_pair);
                    panic!("Found mismatch pairs");
                }
                let second = current_pair.pop().unwrap();
                let first = current_pair.pop().unwrap();
                packet_pairs.push((first, second));
            },
            _ => {
                current_pair.push(parse_packet(line));
            }
        }
    }
                if current_pair.len() != 2 {
                    dbg!(current_pair);
                    panic!("Found mismatch pairs");
                }
                let second = current_pair.pop().unwrap();
                let first = current_pair.pop().unwrap();
                packet_pairs.push((first, second));
    packet_pairs
}

fn parse_packet(line: &str) -> Packet {
    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        match c {
            '[' => return parse_list(&mut chars),
            _ => panic!("invalid format")
        }
    }
    panic!("invalid format")
}

fn parse_list(chars: &mut Chars) -> Packet {
    let mut packets: Vec<Packet> = Vec::new();
    let mut next_num = String::new();
    while let Some(c) = chars.next() {
        match c {
            '[' => packets.push(parse_list(chars)),
            num if num.is_numeric() => next_num.push(c),
            ',' => {
                if next_num.len() > 0 {
                    packets.push(Packet::Num(next_num.parse::<u16>().unwrap()));
                    next_num.clear();
                }
            },
            ']' => {
                if next_num.len() > 0 {
                    packets.push(Packet::Num(next_num.parse::<u16>().unwrap()));
                    next_num.clear();
                }
                break
            },
            _ => panic!("Unexpected character while parsing list")
        };
    }
    Packet::List(packets)
}

#[derive(Debug)]
enum Packet {
    Num(u16),
    List(Vec<Packet>)
}

impl Ord for Packet {

    fn cmp(&self, other: &Self) -> Ordering {

        match (self, other) {
            (Packet::Num(num), Packet::Num(other)) => {
                if num < other {
                    Ordering::Less
                } else if num > other {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            },
            (Packet::Num(n), Packet::List(_)) => Packet::List(vec![Packet::Num(*n)]).cmp(other),
            (Packet::List(_), Packet::Num(n)) => self.cmp(&Packet::List(vec![Packet::Num(*n)])),
            (Packet::List(self_list), Packet::List(other_list)) => compare_lists(self_list, other_list)
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,

            (Packet::Num(n), Packet::List(_)) => Packet::List(vec![Packet::Num(*n)]).eq(other),
            (Packet::List(_), Packet::Num(n)) => self.eq(&Packet::List(vec![Packet::Num(*n)])),
            (Packet::List(self_list), Packet::List(other_list)) => compare_lists(self_list, other_list).eq(&Ordering::Equal)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

fn compare_lists(first: &Vec<Packet>, second: &Vec<Packet>) -> Ordering {

    let mut first = first.iter();
    let mut second = second.iter();

    //Need to manually zip because the stdlib's zip won't tell us if self ran out or other ran out
    loop {
        let (first, second) = (first.next(), second.next());
        //dbg!(self, other);
        let result = match (first, second) {
            (Some(first), Some(second)) => first.cmp(second),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => return Ordering::Equal
        };

        match result {
            Ordering::Equal => continue,
            other => return other
        }
    }
}

#[test]
fn test_1() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    let result = find_score_correctly_ordered(input.as_bytes());

    assert_eq!(result, 13);
}

#[test]
fn test_2() {
    let input = "[[],[],[[],3,[[9,3,3],1]]]
[[],[[[4,1,4,10],[7,10,8,8],[7]]],[7,9,0]]";

    let result = find_score_correctly_ordered(input.as_bytes());

    assert_eq!(result, 1);
}

#[test]
fn test_part_2() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    let result = get_decoder_key(input.as_bytes());

    assert_eq!(result, 140);
}