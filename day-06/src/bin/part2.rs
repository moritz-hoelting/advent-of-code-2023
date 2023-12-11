use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult, Parser,
};
use rayon::prelude::*;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u64 {
    separated_pair(time_parser, newline, distance_parser)
        .map(|(time, distance)| calc_game(time, distance))
        .parse(input)
        .unwrap()
        .1
}

fn time_parser(i: &str) -> IResult<&str, u64> {
    preceded(pair(tag("Time:"), space1), separated_list1(space1, digit1))
        .map(|strs| strs.join("").parse::<u64>().expect("Invalid digit"))
        .parse(i)
}

fn distance_parser(i: &str) -> IResult<&str, u64> {
    preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, digit1),
    )
    .map(|strs| strs.join("").parse::<u64>().expect("Invalid digit"))
    .parse(i)
}

fn calc_game(time: u64, distance: u64) -> u64 {
    (1..time)
        .into_par_iter()
        .filter_map(|t| (calc_distance(t, time) > distance).then_some(()))
        .count() as u64
}

fn calc_distance(hold_time: u64, total_time: u64) -> u64 {
    hold_time * (total_time - hold_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(indoc!(
                "
            Time:      7  15   30
            Distance:  9  40  200
            "
            )),
            71503
        );
    }
}
