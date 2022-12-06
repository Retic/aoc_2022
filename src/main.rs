mod day_1;
use crate::day_1::{solve_day_1_1, solve_day_1_2};

fn main() {
    let puzzle_input_day_1 = include_str!("../puzzle_1.txt");

    //Day 1.1
    println!("{}", solve_day_1_1(puzzle_input_day_1));

    //Day 1.2
    println!("{}", solve_day_1_2(puzzle_input_day_1));
}
