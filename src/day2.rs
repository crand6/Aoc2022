/// Library for Day 2 solutions
 
/// Enum for the game choices in each column of the text file input
#[derive(Debug, PartialEq)]
enum Choice {
    /// Column 1 - Rock
    Rock,
    /// Column 1 - Paper
    Paper,
    /// Column 1 - Scissors
    Scissors,
    /// Column 2 - Lose
    Lose,
    /// Column 2 - Draw
    Draw,
    /// Column 2 - Win
    Win
}

/// Represent the solution for a single line of the text file input
#[derive(Debug)]
struct Round {
    /// Whether the opponent has selected Rock, Paper, or Scissors
    opponent: Choice,
    /// Whether the Player has selected Rock, Paper, or Scissors; or Lose, Draw, Win
    player: Choice
}

impl Round {
    /// Solutions for Part A, where X/Y/Z represent choosing Rock, Paper, or Scissors
    fn score_a(&self) -> usize {
        let mut score: usize = match self.player {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
            _ => panic!("Invalid choice in self.player")
        };

        if self.player == self.opponent {
            // Draw
            score = score.checked_add(3).unwrap();
        }
        else if 
            self.player == Choice::Rock && self.opponent == Choice::Scissors
            || self.player == Choice::Paper && self.opponent == Choice::Rock
            || self.player == Choice::Scissors && self.opponent == Choice::Paper {
            score = score.checked_add(6).unwrap();
        }
        else {
            // else, no points for losing
        }

        score
    }

    /// Solution for Part B, where X/Y/Z represent desired Lose, Draw, or Win outcome
    fn score_b(&self) -> usize {
        let mut score: usize= match self.player {
            Choice::Lose => 0,
            Choice::Draw => 3,
            Choice::Win => 6,
            _ => panic!("Invalid choice in self.player")
        };

        score = score.checked_add(
            match *self {
                Self {opponent: Choice::Rock, player: Choice::Lose}
                | Self {opponent: Choice::Paper, player: Choice::Win} 
                | Self {opponent: Choice::Scissors, player: Choice::Draw}
                    => 3,
                Self {opponent: Choice::Rock, player: Choice::Draw}
                | Self {opponent: Choice::Paper, player: Choice::Lose}
                | Self {opponent: Choice::Scissors, player: Choice::Win}
                    => 1,
                Self {opponent: Choice::Paper, player: Choice::Draw}
                | Self {opponent: Choice::Rock, player: Choice::Win}
                | Self {opponent: Choice::Scissors, player: Choice::Lose}
                    => 2,
                _ => unimplemented!("oops")
            }
        ).unwrap();

        if self.player == self.opponent {
            // Draw
            score = score.checked_add(3).unwrap();
        }
        else if
            self.player == Choice::Rock && self.opponent == Choice::Scissors
            || self.player == Choice::Paper && self.opponent == Choice::Rock
            || self.player == Choice::Scissors && self.opponent == Choice::Paper {
            score = score.checked_add(6).unwrap();
        }
        else {
            // else, no points for losing
        }

        score
    }

}

/// Solutions for Part A, where X/Y/Z represent choosing Rock, Paper, or Scissors
pub fn a(data: &str) {
    let mut rounds = vec![];
    for line in data.lines() {
        let mut iter = line.split_whitespace();
        let opponent = match iter.next().unwrap() {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            x => panic!("Invalid first column value {x}")
        };
        let player = match iter.next().unwrap() {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            x => panic!("Invalid second column value {x}")
        };
        let round = Round {
            opponent,
            player
        };
        rounds.push(round);
    }
    let total_score: usize = rounds.iter().map(Round::score_a).sum();
    println!("Total score: {total_score}");
}

/// Solution for Part B, where X/Y/Z represent desired Lose, Draw, or Win outcome
pub fn b(data: &str) {
    let mut rounds = vec![];
    for line in data.lines() {
        let mut iter = line.split_whitespace();
        let opponent = match iter.next().unwrap() {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            x => panic!("Invalid first column value {x}")
        };
        let player = match iter.next().unwrap() {
            "X" => Choice::Lose,
            "Y" => Choice::Draw,
            "Z" => Choice::Win,
            x => panic!("Invalid second column value {x}")
        };
        let round = Round {
            opponent,
            player
        };
        rounds.push(round);
    }
    let total_score: usize = rounds.iter().map(Round::score_b).sum();
    println!("Total score: {total_score}");
}