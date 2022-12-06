pub fn solve_day_1_1(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut current_sum = 0;
    let mut elves = Vec::new();

    for line in lines {
        if line != "" {
            current_sum += line.parse::<i32>().unwrap();
        } else {
            elves.push(current_sum);
            current_sum = 0;
        }
    }
    elves.iter().max().copied().unwrap_or(0)
}

pub fn solve_day_1_2(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut current_sum = 0;
    let mut elves = Vec::new();

    for line in lines {
        if line != "" {
            current_sum += line.parse::<i32>().unwrap();
        } else {
            elves.push(current_sum);
            current_sum = 0;
        }
    }
    elves.sort();

    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elve_calorie_count() {
        let data = "1000\n2000\n3000\r\n\r\n4000\n5000\n6000\r\n\r\n7000\n8000\n9000\r\n\r\n10000";
        assert_eq!(solve_day_1_1(data), 24000)
    }

    #[test]
    fn test_elve_calorie_count_of_top_3() {
        let data = "1000\n2000\n3000\r\n\r\n4000\n5000\n6000\r\n\r\n7000\n8000\n9000\r\n\r\n10000";
        assert_eq!(solve_day_1_2(data), 45000)
    }
}
