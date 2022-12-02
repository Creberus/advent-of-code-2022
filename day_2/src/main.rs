use std::error::Error;
use std::io;

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

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

fn fight(opponent: Opponent, player: Player) -> u32 {
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

fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut score: u32 = 0;

    for line in lines {
        let l = line.unwrap();

        let mut it = l.chars();

        let opponent: Opponent = Opponent::from(it.next().expect("Invalid Opponent input"));
        let _ = it.next();
        let player: Player = Player::from(it.next().expect("Invalid Player input."));

        let round = fight(opponent, player);

        score += round;
    }

    println!("Total score: {}", score);

    Ok(())
}
