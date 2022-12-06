mod day_1;
use crate::day_1::solve_day_1_1;

fn main() {
    let puzzle_input_day_1 = include_str!("../puzzle_1.txt");

    //Day 1.1
    println!("{}", solve_day_1_1(puzzle_input_day_1));
}
