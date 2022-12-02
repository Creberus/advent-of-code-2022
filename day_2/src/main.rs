use std::error::Error;
use std::io;

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

/// Opponent
/// A => Rock
/// B => Paper
/// C => Scissors
enum Opponent {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Opponent {
    fn from(c: char) -> Self {
        match c {
            'A' => Opponent::Rock,
            'B' => Opponent::Paper,
            'C' => Opponent::Scissors,
            _ => panic!("Opponent input is invalid"),
        }
    }
}

/// Player
/// Y => Paper
/// X => Rock
/// Z => Scissors

enum Player {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Player {
    fn from(c: char) -> Self {
        match c {
            'X' => Player::Rock,
            'Y' => Player::Paper,
            'Z' => Player::Scissors,
            _ => panic!("Opponent input is invalid"),
        }
    }
}

/// Player
/// Y => DRAW
/// X => LOSE
/// Z => WIN

enum RoundStatus {
    LOSE,
    DRAW,
    WIN,
}

impl From<char> for RoundStatus {
    fn from(c: char) -> Self {
        match c {
            'X' => RoundStatus::LOSE,
            'Y' => RoundStatus::DRAW,
            'Z' => RoundStatus::WIN,
            _ => panic!("Opponent input is invalid"),
        }
    }
}

fn fight_part_one(opponent: Opponent, player: Player) -> u32 {
    match opponent {
        Opponent::Rock => match player {
            Player::Rock => 1 + DRAW,
            Player::Paper => 2 + WIN,
            Player::Scissors => 3 + LOSE,
        },
        Opponent::Paper => match player {
            Player::Rock => 1 + LOSE,
            Player::Paper => 2 + DRAW,
            Player::Scissors => 3 + WIN,
        },
        Opponent::Scissors => match player {
            Player::Rock => 1 + WIN,
            Player::Paper => 2 + LOSE,
            Player::Scissors => 3 + DRAW,
        },
    }
}

fn fight_part_two(opponent: Opponent, round_status: RoundStatus) -> u32 {
    match opponent {
        Opponent::Rock => match round_status {
            RoundStatus::WIN => WIN + PAPER,
            RoundStatus::DRAW => DRAW + ROCK,
            RoundStatus::LOSE => LOSE + SCISSORS,
        },
        Opponent::Paper => match round_status {
            RoundStatus::WIN => WIN + SCISSORS,
            RoundStatus::DRAW => DRAW + PAPER,
            RoundStatus::LOSE => LOSE + ROCK,
        },
        Opponent::Scissors => match round_status {
            RoundStatus::WIN => WIN + ROCK,
            RoundStatus::DRAW => DRAW + SCISSORS,
            RoundStatus::LOSE => LOSE + PAPER,
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut score: u32 = 0;

    for line in lines {
        let l = line.unwrap();

        let mut it = l.chars();

        // Part 1
        /*
        let opponent: Opponent = Opponent::from(it.next().expect("Invalid Opponent input"));
        let _ = it.next();
        let player: Player = Player::from(it.next().expect("Invalid Player input."));

        let round = fight_part_one(opponent, player);
        */

        // Part 2
        let opponent: Opponent = Opponent::from(it.next().expect("Invalid Opponent input"));
        let _ = it.next();
        let round_status: RoundStatus =
            RoundStatus::from(it.next().expect("Invalid Player input."));

        let round = fight_part_two(opponent, round_status);

        score += round;
    }

    println!("Total score: {}", score);

    Ok(())
}
