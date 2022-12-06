mod day_1;
mod day_2;
use crate::day_1::{solve_day_1_1, solve_day_1_2};
use crate::day_2::solve_day_2_1;

fn main() {
    let puzzle_input_day_1 = include_str!("../puzzle_1.txt");
    let puzzle_input_day_2 = include_str!("../puzzle_2.txt");

    //Day 1.1
    println!("{}", solve_day_1_1(puzzle_input_day_1));

    //Day 1.2
    println!("{}", solve_day_1_2(puzzle_input_day_1));

    //Day 2.1
    println!("{}", solve_day_2_1(puzzle_input_day_2));
}
