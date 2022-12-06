use y2022::day5::day5::Scenario;

use crate::y2022::day1;
use crate::y2022::day2;
use crate::y2022::day3::day3 as day3;
use crate::y2022::day4::day4 as day4;
use crate::y2022::day5::day5 as day5;

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
    day5::solve_part("day5.txt", Scenario::One);
    day5::solve_part("day5.txt", Scenario::Two);
}