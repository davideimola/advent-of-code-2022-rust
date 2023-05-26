use nom::character::complete::{alphanumeric1, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn main() {
    let input = include_str!("./data.txt");
    let (res, priorities) = find_total_priorities(input).unwrap();
    assert_eq!(res, "");

    println!("Total priorities: {}", priorities);

    let (res, priorities) = find_total_priorities_for_3_badges(input).unwrap();
    assert_eq!(res, "");

    println!("Total priorities for 3 badges: {}", priorities);
}

fn char_to_u32(c: &char) -> u32 {
    if *c >= 'A' && *c <= 'Z' {
        return *c as u32 - 'A' as u32 + 27;
    }

    *c as u32 - 'a' as u32 + 1
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, line) = alphanumeric1(input)?;
    let mid = line.len() / 2;
    let left = &line[..mid];
    let right = &line[mid..];
    Ok((input, (left, right)))
}

fn parse_line_score(input: &str) -> IResult<&str, u32> {
    let (input, (l, r)) = parse_line(input)?;
    for c in l.chars() {
        if r.contains(c) {
            return Ok((input, char_to_u32(&c)));
        }
    }
    Ok((input, 0))
}

fn find_total_priorities(input: &str) -> IResult<&str, u32> {
    let (input, priorities) = separated_list1(newline, parse_line_score)(input)?;
    Ok((input, priorities.iter().sum()))
}

fn parse_3_badges(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (badge1, _)) = tuple((alphanumeric1, newline))(input)?;
    let (input, (badge2, _)) = tuple((alphanumeric1, newline))(input)?;
    let (input, badge3) = alphanumeric1(input)?;
    Ok((input, (badge1, badge2, badge3)))
}

fn parse_3_elf_badge(input: &str) -> IResult<&str, u32> {
    let (input, (badge1, badge2, badge3)) = parse_3_badges(input)?;
    for c in badge1.chars() {
        if badge2.contains(c) && badge3.contains(c) {
            return Ok((input, char_to_u32(&c)));
        }
    }
    Ok((input, 0))
}

fn find_total_priorities_for_3_badges(input: &str) -> IResult<&str, u32> {
    let (input, priorities) = separated_list1(newline, parse_3_elf_badge)(input)?;
    Ok((input, priorities.iter().sum()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_priorities() {
        let input = include_str!("./sample.txt");
        let (res, priorities) = find_total_priorities(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(priorities, 157)
    }

    #[test]
    fn total_priorities_for_3_badges() {
        let input = include_str!("./sample.txt");
        let (res, priorities) = find_total_priorities_for_3_badges(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(priorities, 70)
    }
}