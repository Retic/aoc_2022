const PAPER: i32 = 2;
const ROCK: i32 = 1;
const SCISSORS: i32 = 3;
const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

fn calculate_score(player_move: &str, opponent_move: &str) -> i32 {
    let mut score = 0;
    //let paper = 2;

    match opponent_move {
        "rock" => {
            //Rock
            match player_move {
                "rock" => {
                    score += ROCK;
                    score += DRAW;
                }
                "paper" => {
                    score += PAPER;
                    score += WIN;
                }
                "scissors" => {
                    score += SCISSORS;
                    score += LOSS;
                }
                _ => {}
            }
        }
        "paper" => match player_move {
            "rock" => {
                score += ROCK;
                score += LOSS;
            }
            "paper" => {
                score += PAPER;
                score += DRAW;
            }
            "scissors" => {
                score += SCISSORS;
                score += WIN;
            }
            _ => {}
        },
        "scissors" => match player_move {
            "rock" => {
                score += ROCK;
                score += WIN;
            }
            "paper" => {
                score += PAPER;
                score += LOSS;
            }
            "scissors" => {
                score += SCISSORS;
                score += DRAW;
            }
            _ => {}
        },
        _ => {}
    }

    score
}

pub fn solve_day_2_1(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut score = 0;

    for line in lines {
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

        score += calculate_score(player_move, opponent_move);
    }

    score
}

pub fn solve_day_2_2(puzzle_input: &str) -> i32 {
    let lines = puzzle_input.lines().map(|l| l);
    let mut score = 0;

    for line in lines {
        let result: Vec<&str> = line.split(" ").collect();
        let opponent_move = match result[0] {
            "A" => "rock",
            "B" => "paper",
            "C" => "scissors",
            _ => "",
        };
        let player_move = match result[1] {
            "X" => {
                //LOSS
                match opponent_move {
                    "rock" => "scissors",
                    "paper" => "rock",
                    "scissors" => "paper",
                    _ => "",
                }
            }
            "Y" => {
                //DRAW
                opponent_move
            }
            "Z" => {
                //WIN
                match opponent_move {
                    "rock" => "paper",
                    "paper" => "scissors",
                    "scissors" => "rock",
                    _ => "",
                }
            }
            _ => "",
        };

        score += calculate_score(player_move, opponent_move);
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

    #[test]
    fn test_rock_paper_scissors_with_end_goal() {
        let data = "A Y\nB X\nC Z";
        assert_eq!(solve_day_2_2(data), 12)
    }
}
