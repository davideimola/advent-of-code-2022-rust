use nom::bytes::complete::tag;
use nom::IResult;
use nom::sequence::separated_pair;

mod command;
mod board;

use board::Board;
use command::Command;

fn main() {
    let input = include_str!("./data.txt");
    let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();

    for command in commands {
        board.execute(&command);
    }

    println!("First row: {}", board.first_row());

    let input = include_str!("./data.txt");
    let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();

    for command in commands {
        board.execute_v2(&command);
    }

    println!("First row: {}", board.first_row());
}

fn parse_board_and_commands(input: &str) -> IResult<&str, (Board, Vec<Command>)> {
    separated_pair(Board::parse, tag("\n\n"), Command::parse_many)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_crate() {
        let input = include_str!("./sample.txt");
        let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();

        for command in commands {
            board.execute(&command);
        }

        assert_eq!("CMZ", board.first_row());
    }
}