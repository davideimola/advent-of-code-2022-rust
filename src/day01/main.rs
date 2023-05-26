use std::cmp::Ordering;

use nom::character::complete;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn main() {
    let input = include_str!("./data.txt");
    let (res, elves) = parse_elves(input).unwrap();
    assert_eq!(res, "");

    let result = find_elf_with_max_calories(&elves).unwrap();
    println!("Max calories: {}", result.total_calories());

    println!("Sum of top 3 calories: {}", find_top_3_max_calories(&elves).unwrap());
}

fn find_elf_with_max_calories(elves: &[Elf]) -> Option<&Elf> {
    elves.iter().max_by(|a, b| a.cmp(b))
}

fn find_top_3_max_calories(elves: &[Elf]) -> Option<u32> {
    let mut c = elves.to_vec();
    c.sort_by(|a, b| b.cmp(a));

    let res = c[0..3].iter().map(|e| e.total_calories()).sum();
    Some(res)
}

#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, calories) = separated_list1(newline, Elf::parse_calories)(s)?;
        Ok((s, Elf { calories }))
    }

    fn parse_calories(s: &str) -> IResult<&str, u32> {
        complete::u32(s)
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

fn parse_elves(s: &str) -> IResult<&str, Vec<Elf>> {
    let double_new_line = tuple((newline, newline));
    separated_list1(double_new_line, Elf::parse)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_elf() {
        let input = include_str!("./sample.txt");
        let (res, elves) = parse_elves(input).unwrap();
        assert_eq!(res, "");

        let result = find_elf_with_max_calories(&elves).unwrap();
        assert_eq!(result.total_calories(), 24000);
    }

    #[test]
    fn top_3_elves() {
        let input = include_str!("./sample.txt");
        let (res, elves) = parse_elves(input).unwrap();
        assert_eq!(res, "");

        let result = find_top_3_max_calories(&elves).unwrap();
        assert_eq!(result, 45000);
    }
}