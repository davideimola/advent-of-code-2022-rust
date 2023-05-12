use nom::bytes::streaming::tag;
use nom::character::complete::{anychar, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    let input = include_str!("./data.txt");
    let (res, games) = parse_games(input).unwrap();
    assert_eq!(res, "");

    let total_score = games.iter().map(|(a,b)| b.game_score(a)).sum::<i32>();
    println!("Total score: {}", total_score);

    let (res, games) = parse_games_v2(input).unwrap();
    assert_eq!(res, "");

    let total_score = games.iter().map(|(a,b)| b.game_score(a)).sum::<i32>();
    println!("Total score V2: {}", total_score);
}

#[derive(Copy,Debug,Clone,PartialEq)]
enum PlayChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy,Debug,Clone,PartialEq)]
enum GameOutcome {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

impl GameOutcome {
    fn score(&self) -> i32 {
        *self as i32
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'X' => Ok((input, GameOutcome::Lose)),
            'Y' => Ok((input, GameOutcome::Tie)),
            'Z' => Ok((input, GameOutcome::Win)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }
}

impl PlayChoice {
    fn game_score(&self, other: &Self) -> i32 {
        self.game(other).score() + self.score()
    }

    pub fn score(&self) -> i32 {
        *self as i32
    }

    fn game(&self, other: &Self) -> GameOutcome {
        match (self, other) {
            (PlayChoice::Rock, PlayChoice::Rock) => GameOutcome::Tie,
            (PlayChoice::Rock, PlayChoice::Paper) => GameOutcome::Lose,
            (PlayChoice::Rock, PlayChoice::Scissors) => GameOutcome::Win,
            (PlayChoice::Paper, PlayChoice::Rock) => GameOutcome::Win,
            (PlayChoice::Paper, PlayChoice::Paper) => GameOutcome::Tie,
            (PlayChoice::Paper, PlayChoice::Scissors) => GameOutcome::Lose,
            (PlayChoice::Scissors, PlayChoice::Rock) => GameOutcome::Lose,
            (PlayChoice::Scissors, PlayChoice::Paper) => GameOutcome::Win,
            (PlayChoice::Scissors, PlayChoice::Scissors) => GameOutcome::Tie,
        }
    }

    fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        separated_pair(PlayChoice::parse, tag(" "), PlayChoice::parse_second)(input)
    }

    fn parse_pair_v2(input: &str) -> IResult<&str, (Self, Self)> {
        let (input, a) = PlayChoice::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, outcome) = GameOutcome::parse(input)?;

        let b = a.response_for_outcome(outcome);

        Ok((input, (a, b)))
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'A' => Ok((input, PlayChoice::Rock)),
            'B' => Ok((input, PlayChoice::Paper)),
            'C' => Ok((input, PlayChoice::Scissors)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }

    fn parse_second(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'X' => Ok((input, PlayChoice::Rock)),
            'Y' => Ok((input, PlayChoice::Paper)),
            'Z' => Ok((input, PlayChoice::Scissors)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }

    fn response_for_outcome(&self, outcome: GameOutcome) -> Self {
        match outcome {
            GameOutcome::Tie => *self,
            GameOutcome::Win =>  match self {
                PlayChoice::Rock => PlayChoice::Paper,
                PlayChoice::Paper => PlayChoice::Scissors,
                PlayChoice::Scissors => PlayChoice::Rock,
            },
            GameOutcome::Lose => match self {
                PlayChoice::Rock => PlayChoice::Scissors,
                PlayChoice::Paper => PlayChoice::Rock,
                PlayChoice::Scissors => PlayChoice::Paper,
            }
        }
    }
}

fn parse_games(input: &str) -> IResult<&str, Vec<(PlayChoice,PlayChoice)>> {
    separated_list1(newline, PlayChoice::parse_pair)(input)
}

fn parse_games_v2(input: &str) -> IResult<&str, Vec<(PlayChoice, PlayChoice)>> {
    separated_list1(newline, PlayChoice::parse_pair_v2)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_score() {
        let input = include_str!("./sample.txt");

        let (res, games) = parse_games(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(games.len(), 3);
        assert_eq!(games[0], (PlayChoice::Rock, PlayChoice::Paper));

        let total_score = games.iter().map(|(a,b)| b.game_score(a)).sum::<i32>();
        assert_eq!(total_score, 15);
    }

    #[test]
    fn total_score_2() {
        let input = include_str!("./sample.txt");

        let (res, games) = parse_games_v2(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(games.len(), 3);
        assert_eq!(games[0], (PlayChoice::Rock, PlayChoice::Rock));

        let total_score = games.iter().map(|(a,b)| b.game_score(a)).sum::<i32>();
        assert_eq!(total_score, 12);
    }
}