use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u64 {
    separated_pair(time_parser, newline, distance_parser)
        .map(|(times, distances)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| calc_game(time, distance) as u64)
                .product::<u64>()
        })
        .parse(input)
        .unwrap()
        .1
}

fn time_parser(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        pair(tag("Time:"), space1),
        separated_list1(space1, complete::u32),
    )(i)
}

fn distance_parser(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, complete::u32),
    )(i)
}

fn calc_game(time: u32, distance: u32) -> u32 {
    (1..time)
        .filter_map(|t| (calc_distance(t, time) > distance).then_some(()))
        .count() as u32
}

fn calc_distance(hold_time: u32, total_time: u32) -> u32 {
    hold_time * (total_time - hold_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
                "
        Time:      7  15   30
        Distance:  9  40  200
        "
            )),
            288
        );
    }
}
