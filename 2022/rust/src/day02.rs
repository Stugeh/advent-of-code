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

pub struct Game {
    opponent: Choice,
    player: Choice,
}

impl Game {
    fn char_to_choice(ch: char) -> Choice {
        match ch {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => Choice::None,
        }
    }

    pub fn parse_game(string: &str) -> Game {
        let opponent_char = string.chars().next().unwrap();
        let player_char = string.chars().nth(2).unwrap();

        Game {
            opponent: Game::char_to_choice(opponent_char),
            player: Game::char_to_choice(player_char),
        }
    }

    pub fn score_match(&self) -> i32 {
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
        .map(|game| game.score_match())
        .sum();

    println!("{}", total_score)
}
