use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

fn main() {
    let input = include_str!("./data.txt");
    println!("Total full overlaps: {}", total_full_overlaps(input));
    println!("Total partial overlaps: {}", total_partial_overlaps(input));
}

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
        Ok((input, Range { start, end }))
    }

    fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        separated_pair(Range::parse, tag(","), Range::parse)(input)
    }

    fn fully_overlaps(&self, other: &Self) -> bool {
        self.fully_contains(other) || other.fully_contains(self)
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn partial_overlaps(&self, other: &Self) -> bool {
        self.partial_contains(other) || other.partial_contains(self)
    }

    fn partial_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

fn total_full_overlaps(input: &str) -> u32 {
    let (_, ranges) = separated_list1(newline, Range::parse_pair)(input).unwrap();
    ranges
        .iter()
        .filter(|(a, b)| a.fully_overlaps(b))
        .count() as u32
}

fn total_partial_overlaps(input: &str) -> u32 {
    let (_, ranges) = separated_list1(newline, Range::parse_pair)(input).unwrap();
    ranges
        .iter()
        .filter(|(a, b)| a.partial_overlaps(b))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_overlaps_count() {
        let input = include_str!("./sample.txt");
        assert_eq!(total_full_overlaps(input), 2);
    }

    #[test]
    fn partial_overlaps_count() {
        let input = include_str!("./sample.txt");
        assert_eq!(total_partial_overlaps(input), 4);
    }
}