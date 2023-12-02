use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u32 {
    input.lines().map(game).sum()
}

fn game(input: &str) -> u32 {
    let (_, product) = tuple((id, all_results))(input).unwrap().1;

    product
}

fn digit_u32(i: &str) -> IResult<&str, u32> {
    map(digit1, |d: &str| d.parse().unwrap())(i)
}

fn id(i: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), digit_u32, tag(": "))(i)
}

fn all_results(i: &str) -> IResult<&str, u32> {
    map(separated_list1(tag("; "), one_result), |res| {
        let (red, green, blue) =
            res.iter()
                .flatten()
                .fold(
                    (0_u32, 0_u32, 0_u32),
                    |(r, g, b), (amount, color)| match color {
                        Color::Red => (r.max(*amount), g, b),
                        Color::Green => (r, g.max(*amount), b),
                        Color::Blue => (r, g, b.max(*amount)),
                    },
                );

        red * green * blue
    })(i)
}

fn one_result(i: &str) -> IResult<&str, Vec<(u32, Color)>> {
    separated_list1(tag(", "), color)(i)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

fn color(i: &str) -> IResult<&str, (u32, Color)> {
    pair(
        terminated(digit_u32, char(' ')),
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286)
    }
}
