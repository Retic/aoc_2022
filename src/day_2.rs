pub fn solve_day_2_1(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut score = 0;

    for line in lines {
        let paper = 2;
        let rock = 1;
        let scissors = 3;
        let win = 6;
        let draw = 3;
        let loss = 0;

        let result: Vec<&str> = line.split(" ").collect();
        let opponent_move = match result[0] {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => "",
        };
        let player_move = match result[1] {
            "X" => "rock",
            "Y" => "paper",
            "Z" => "scissors",
            _ => "",
        };
        match opponent_move {
            "rock" => {
                //Rock
                match player_move {
                    "rock" => {
                        score += rock;
                        score += draw;
                    }
                    "paper" => {
                        score += paper;
                        score += win;
                    }
                    "scissors" => {
                        score += scissors;
                        score += loss;
                    }
                    _ => {}
                }
            }
            "paper" => match player_move {
                "rock" => {
                    score += rock;
                    score += loss;
                }
                "paper" => {
                    score += paper;
                    score += draw;
                }
                "scissors" => {
                    score += scissors;
                    score += win;
                }
                _ => {}
            },
            "scissors" => match player_move {
                "rock" => {
                    score += rock;
                    score += win;
                }
                "paper" => {
                    score += paper;
                    score += loss;
                }
                "scissors" => {
                    score += scissors;
                    score += draw;
                }
                _ => {}
            },
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod test2s {
    use super::*;

    #[test]
    fn test_rock_paper_scissors() {
        let data = "A Y\nB X\nC Z";
        assert_eq!(solve_day_2_1(data), 15)
    }
}
