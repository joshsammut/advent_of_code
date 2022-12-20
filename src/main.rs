use std::fs::File;

use y2022::day10;
use y2022::day10::render_crt;
use y2022::day11;
use y2022::day12;
use y2022::day13;
use y2022::day5::day5::Scenario;

use crate::y2022::day1;
use crate::y2022::day2;
use crate::y2022::day3::day3 as day3;
use crate::y2022::day4::day4 as day4;
use crate::y2022::day5::day5 as day5;
use crate::y2022::day6::day6 as day6;
use crate::y2022::day7::day7 as day7;
use crate::y2022::day8;
use crate::y2022::day9;

mod y2022;

fn main() {
    // day1::day1::solve_part1("day1.txt");
    // day1::day1::solve_part2("day1.txt");
    //day2::day2::solve_part1("day2.txt");
    // day2::day2::solve_part2("day2.txt");
    // day3::solve_part1("day3.txt");
    // day3::solve_part2("day3.txt");
    // day4::solve_part1("day4.txt");
    // day4::solve_part2("day4.txt");
    // day5::solve_part("day5.txt", Scenario::One);
    // day5::solve_part("day5.txt", Scenario::Two);
    // day6::solve("day6.txt");
    // day7::part1("day7.txt");
    //day7::part2("day7.txt");
    // day8::part_1("day8.txt");
    // day8::part_2("day8.txt");
    //day9::part_1("day9.txt");
    //day9::part_2("day9.txt");
    //day10::get_total_signal_strenth(File::open("day10.txt").unwrap());
    //day10::render_crt("day10.txt");
    //day11::find_busy_monkeys(File::open("day11.txt").unwrap(), 20, 3);
    //day11::find_busy_monkeys(File::open("day11.txt").unwrap(), 10000, 1); //doesn't work
    //day12::find_shortest_from_start(File::open("day12.txt").unwrap());
    //day12::find_any_shortest(File::open("day12.txt").unwrap());
    //day13::find_score_correctly_ordered(File::open("day13.txt").unwrap());
    day13::get_decoder_key(File::open("day13.txt").unwrap());
}