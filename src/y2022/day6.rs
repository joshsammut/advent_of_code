pub mod day6 {
    use std::{io::Read, str::from_utf8, collections::{HashSet, VecDeque}, fs};

    pub fn solve(filename: &str) {
        let input = fs::read_to_string(filename).unwrap();
        let start_of_packet = signal_start(input.as_str(), 4);
        let start_of_message = signal_start(input.as_str(), 14);
        println!("start of signal after {} characters, start of message after {}", start_of_packet, start_of_message);
    }

    #[derive(Debug)]
    struct SignalAnalyzer {
        seen_chars_set: HashSet<String>,
        seen_chars: VecDeque<String>
    }

    impl SignalAnalyzer {
        fn new() -> SignalAnalyzer {
            SignalAnalyzer { seen_chars_set: HashSet::new(), seen_chars: VecDeque::new() }
        }

        /**
         * Returns true if this is the nth unique character we've seen in a row
         */
        fn is_nth_unique(&self, character: &str, num: usize) -> bool {
            self.seen_chars_set.len() == num - 1 &&
                !self.seen_chars_set.contains(character)
        }

        /**
         * Adds a new character we've seen, only allowing up to `max_chars` to be recorded
         */
        fn add_character(&mut self, character: String, max_chars: usize) {
            //We've seen this character before, evict all characters up to that point
            let character = &character;
            if self.seen_chars_set.contains(character) {
                //Remove characters from the seen list in order we saw them until we
                //we hit this character
                while let Some(evicted) = self.seen_chars.pop_front() {
                    self.seen_chars_set.remove(&evicted);
                    if &evicted == character {
                        break;
                    }
                }
            }

            //Now let's add this character
            self.seen_chars_set.insert(character.to_string());
            self.seen_chars.push_back(character.to_string());
        }
    }

    fn signal_start(input: &str, num_chars: usize) -> u32 {

        let mut analyzer = SignalAnalyzer::new();
        for (index, &character) in input.as_bytes().iter().enumerate() {
            let character = &[character];
            let character = from_utf8(character).unwrap();

            if analyzer.is_nth_unique(character, num_chars) {
                //0 indexed so we'll add 1
                return (index + 1) as u32;
            } else {
                analyzer.add_character(character.to_string(), num_chars);
            }
        }
        panic!("No start of string indicator found");
    }

    #[test]
    fn test_1() {
        let test_cases = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 5),
            ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 11)
        ];
        for (index, (input, expected)) in test_cases.iter().enumerate() {
            println!("Test case {}", index);
            let result = signal_start(input, 4);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_2() {
        let test_cases = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 23),
            ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 26)
        ];
        for (index, (input, expected)) in test_cases.iter().enumerate() {
            println!("Test case {}", index);
            let result = signal_start(input, 14);
            assert_eq!(result, *expected);
        }
    }
}