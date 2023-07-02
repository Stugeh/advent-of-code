// Day 2: Rock Paper scissors

// COL 1 (what the opponent plays):
// A: Rock
// B: Paper
// C: Scissors

// COL 2 (what we play):
// X: Rock
// Y: Paper
// Z: Scissors

// Shapes give a base score:
// Rock:1, Paper:2, Scissors:3
// Loss:0, Draw:3, Win: 6

// Calculate total score from input file

// possible matches
// rock vs paper
// rock vs scissors
// paper vs scissors

#[derive(Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    None = 0,
}

enum Goal {
    Win,
    Draw,
    Lose,
}

pub struct Game {
    opponent: Choice,
    player: Choice,
}

impl Game {
    pub fn parse_game(string: &str) -> Game {
        let opponent_choice = match string.chars().next().unwrap() {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => Choice::None,
        };

        let player_goal = match string.chars().last().unwrap() {
            'Z' => Goal::Win,
            'Y' => Goal::Draw,
            'X' => Goal::Lose,
            _ => Goal::Draw,
        };

        let player_choice = match player_goal {
            Goal::Draw => opponent_choice.clone(),

            Goal::Win => match opponent_choice {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
                _ => Choice::None,
            },

            Goal::Lose => match opponent_choice {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
                _ => Choice::None,
            },
        };
        Game {
            opponent: opponent_choice,
            player: player_choice,
        }
    }

    pub fn count_score(&self) -> i32 {
        let mut total: i32 = self.player.clone() as i32;

        total += match self.player {
            Choice::Rock => match self.opponent {
                Choice::Rock => 3,
                Choice::Scissors => 6,
                _ => 0,
            },
            Choice::Paper => match self.opponent {
                Choice::Rock => 6,
                Choice::Paper => 3,
                _ => 0,
            },
            Choice::Scissors => match self.opponent {
                Choice::Paper => 6,
                Choice::Scissors => 3,
                _ => 0,
            },
            _ => 0,
        };

        total
    }
}

pub fn get_score(input: Vec<String>) {
    let total_score: i32 = input
        .iter()
        .map(|item| Game::parse_game(item))
        .map(|game| game.count_score())
        .sum();

    println!("{}", total_score)
}
