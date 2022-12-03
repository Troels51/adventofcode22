use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Game {
    opponent: Choice,
    my: Choice,
}

enum GameResult {
    Won,
    Lost,
    Draw,
}

impl Choice {
    fn win(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
    fn lose(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
    fn draw(&self) -> Choice {
        *self
    }
}

impl Game {
    fn score(&self) -> u32 {
        let choice_score = match self.my {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        let result_score = match self.result() {
            GameResult::Won => 6,
            GameResult::Draw => 3,
            GameResult::Lost => 0,
        };
        result_score + choice_score
    }

    fn result(&self) -> GameResult {
        match (&self.my, &self.opponent) {
            (Choice::Rock, Choice::Paper) => GameResult::Lost,
            (Choice::Paper, Choice::Scissors) => GameResult::Lost,
            (Choice::Scissors, Choice::Rock) => GameResult::Lost,

            (Choice::Paper, Choice::Rock) => GameResult::Won,
            (Choice::Scissors, Choice::Paper) => GameResult::Won,
            (Choice::Rock, Choice::Scissors) => GameResult::Won,

            (Choice::Paper, Choice::Paper) => GameResult::Draw,
            (Choice::Scissors, Choice::Scissors) => GameResult::Draw,
            (Choice::Rock, Choice::Rock) => GameResult::Draw,
        }
    }
}

fn main() {
    let path = Path::new("./puzzle_inputs/day2.txt");
    let buffer = BufReader::new(File::open(path).unwrap());
    let games: Vec<Game> = buffer
        .lines()
        .into_iter()
        .filter_map_ok(|line: String| {
            line.split_whitespace()
                .next_tuple()
                .map(|(opp, my)| -> Result<Game, ()> {
                    Ok(Game {
                        opponent: opp.parse()?,
                        my: my.parse()?,
                    })
                })
        })
        .filter_map(|f| f.ok())
        .filter_map(|f| f.ok())
        .collect();
    let score = games.into_iter().fold(0, |acc, game| acc + game.score());
    println!(
        "Part 1: Total score according to strategy guide is {}",
        score
    );

    // Part 2, is done a bit backwards, but only because i wanted to preserve structs from part 1
    let buffer = BufReader::new(File::open(path).unwrap());
    let games_part2: Vec<Game> = buffer
        .lines()
        .into_iter()
        .filter_map_ok(|line: String| {
            line.split_whitespace()
                .next_tuple()
                .map(|(opp, outcome)| -> Result<Game, ()> {
                    let opp: Choice = opp.parse()?;
                    let outcome: GameResult = outcome.parse()?;
                    let my = match outcome {
                        GameResult::Won => opp.win(),
                        GameResult::Lost => opp.lose(),
                        GameResult::Draw => opp.draw(),
                    };
                    Ok(Game {
                        opponent: opp,
                        my: my,
                    })
                })
        })
        .filter_map(|f| f.ok())
        .filter_map(|f| f.ok())
        .collect();
    let score = games_part2
        .into_iter()
        .fold(0, |acc, game| acc + game.score());
    println!(
        "Part 2: Total score according to strategy guide is {}",
        score
    );
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(GameResult::Lost),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Won),
            _ => Err(()),
        }
    }
}
