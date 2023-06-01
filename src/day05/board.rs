use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{anychar, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::command::Command;

#[derive(Debug, PartialEq)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq)]
struct Tile {
    pub val: char,
}

impl Tile {
    pub fn new(val: char) -> Self {
        Tile { val }
    }
}

impl Board {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut parse_rows = separated_list1(newline, parse_row);
        let (input, mut rows) = parse_rows(input)?;

        let (input, _) = newline(input)?;
        let (input, cnt) = parse_index(input)?;

        rows.reverse();

        let mut tiles: Vec<Vec<Tile>> = vec![];
        for i in 0..cnt {
            let mut tile: Vec<Tile> = vec![];
            for row in rows.iter() {
                if let Some(el) = &row[i] {
                    tile.push(Tile::new(el.val));
                }
            }

            tiles.push(tile);
        }

        Ok((input, Self { tiles }))
    }

    pub fn execute(&mut self, command: &Command) {
        for _ in 0..command.size {
            let tile = self.tiles[command.from].pop().unwrap();
            self.tiles[command.to].push(tile);
        }
    }

    pub fn execute_v2(&mut self, command: &Command) {
        let len = self.tiles[command.from].len();
        let from_range = len - command.size;

        let moved: Vec<_> = self.tiles[command.from].splice(from_range..len, vec![]).collect();
        self.tiles[command.to].extend(moved);
    }

    pub fn first_row(&self) -> String {
        let mut result = String::new();
        for tile in self.tiles.iter() {
            if let Some(el) = tile.last() {
                result.push(el.val);
            }
        }

        result
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<Option<Tile>>> {
    separated_list1(tag(" "), parse_element)(input)
}

fn parse_index(input: &str) -> IResult<&str, usize> {
    let (input, list) = separated_list1(tag(" "), tuple((tag(" "), complete::u16, tag(" "))))(input)?;

    Ok((input, list.len()))
}

fn parse_element(input: &str) -> IResult<&str, Option<Tile>> {
    if let Ok((input, el)) = parse_existing(input) {
        return Ok((input, Some(el)));
    }

    match parse_empty(input) {
        Ok((input, _)) => Ok((input, None)),
        Err(e) => Err(e),
    }
}

fn parse_existing(input: &str) -> IResult<&str, Tile> {
    let (input, (_, val, _)) = tuple((tag("["), anychar, tag("]")))(input)?;
    Ok((input, Tile::new(val)))
}

fn parse_empty(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_execution() {
        let mut board = Board {
            tiles: vec![
                vec![Tile::new('a'), Tile::new('b')],
                vec![Tile::new('d')],
                vec![],
            ]
        };

        let command = Command {
            size: 1,
            from: 0,
            to: 1,
        };

        board.execute(&command);

        assert_eq!(board.tiles, vec![
            vec![Tile::new('a')],
            vec![Tile::new('d'), Tile::new('b')],
            vec![],
        ]);
    }

    #[test]
    fn test_command_v2_execution() {
        let mut board = Board {
            tiles: vec![
                vec![Tile::new('a'), Tile::new('b'), Tile::new('c')],
                vec![Tile::new('d')],
                vec![],
            ]
        };

        let command = Command {
            size: 2,
            from: 0,
            to: 1,
        };

        board.execute_v2(&command);

        assert_eq!(board.tiles, vec![
            vec![Tile::new('a')],
            vec![Tile::new('d'), Tile::new('b'), Tile::new('c')],
            vec![],
        ]);
    }
}