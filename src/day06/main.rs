fn main() {
    let input = include_str!("./data.txt");
    let res = parse_marker(input);

    println!("Marker: {:?}", res);

    let res_m = parse_message(input);

println!("Message: {:?}", res_m);
}

fn parse_marker(input: &str) -> Option<usize> {
    parse_marker_for_n_chars(input, 4)
}

fn parse_message(input: &str) -> Option<usize> {
    parse_marker_for_n_chars(input, 14)
}

fn parse_marker_for_n_chars (input: &str, n: usize) -> Option<usize> {
    if input.len() < n {
        return None;
    }

    for i in n-1..input.len() {
        let start = i+1-n;
        if substring_contains_duplicates(&input[start..i+1]) {
            return Some(i+1);
        }
    }

    None
}

fn substring_contains_duplicates(input: &str) -> bool {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.len() == input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_marker() {
        assert_eq!(parse_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(parse_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(parse_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(parse_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(parse_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_parse_message() {
        assert_eq!(parse_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(parse_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(parse_message("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(parse_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(parse_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}