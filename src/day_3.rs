static CHARSET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn solve_day_3_1(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut score = 0;
    for line in lines {
        let split_index = (line.len()) / 2;
        let first_half = &line[..split_index];
        let second_half = &line[split_index..];

        for i in 0..52 {
            if first_half.contains(CHARSET[i]) && second_half.contains(CHARSET[i]) {
                score += i + 1;
            }
        }
    }

    score.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_sort() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_day_3_1(data), 157)
    }
}
