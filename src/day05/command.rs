use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub size: usize,
    pub from: usize,
    pub to: usize,
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (_, size, _, from, _, to)) = tuple((
            tag("move "),
            complete::u32,
            tag(" from "),
            complete::u32,
            tag(" to "),
            complete::u32,
        ))(input)?;
        Ok((
            input,
            Command {
                size: size as usize,
                from: from as usize - 1,
                to: to as usize - 1,
            },
        ))
    }

    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list1(newline, Self::parse)(input)
    }
}
