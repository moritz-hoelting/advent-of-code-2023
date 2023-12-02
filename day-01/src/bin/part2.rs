use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, map_opt, value},
    multi::many1,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> u32 {
    input.lines().map(line).sum()
}

fn line(input: &str) -> u32 {
    let digits = many1(alt((map(digit, Some), value(None, anychar))))(input)
        .unwrap()
        .1;
    let mut digits = digits.iter().filter_map(|d| d.to_owned()).peekable();
    let first = digits.peek().unwrap().to_owned();
    let last = digits.last().unwrap();

    (first * 10) + last
}

fn digit(i: &str) -> IResult<&str, u32> {
    alt((
        map_opt(anychar, |c| c.to_digit(10)),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }

    #[test]
    fn test_line2() {
        assert_eq!(line("two1nine"), 29);
        assert_eq!(line("eightwothree"), 83);
        assert_eq!(line("abcone2threexyz"), 13);
        assert_eq!(line("xtwone3four"), 24);
        assert_eq!(line("4nineeightseven2"), 42);
        assert_eq!(line("zoneight234"), 14);
        assert_eq!(line("7pqrstsixteen"), 76);
    }
}
