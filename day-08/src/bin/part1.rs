use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::{is_a, tag, take_while1},
    character::{
        complete::{char, line_ending},
        is_alphabetic,
    },
    combinator::map,
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u32 {
    let (directions, nodes) = separated_pair(
        map(is_a("RL"), |s: &str| {
            s.chars()
                .map(|c| match c {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        }),
        count(line_ending, 2),
        separated_list1(line_ending, node_parser),
    )(input)
    .expect("invalid input")
    .1;

    let nodes = nodes.into_iter().collect::<HashMap<&str, (&str, &str)>>();

    match directions
        .iter()
        .cycle()
        .enumerate()
        .fold_while(("AAA", 0), |(acc, _), (i, d)| {
            let (l, r) = nodes.get(acc).unwrap();
            let next = match d {
                Direction::Right => r,
                Direction::Left => l,
            };

            if next == &"ZZZ" {
                FoldWhile::Done((next, i))
            } else {
                FoldWhile::Continue((next, i))
            }
        }) {
        FoldWhile::Done((_, i)) => i as u32 + 1,
        FoldWhile::Continue(_) => panic!("invalid input"),
    }
}

fn node_parser(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        take_while1(|c| is_alphabetic(c as u8)),
        tag(" = "),
        delimited(
            char('('),
            separated_pair(
                take_while1(|c| is_alphabetic(c as u8)),
                tag(", "),
                take_while1(|c| is_alphabetic(c as u8)),
            ),
            char(')'),
        ),
    )(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
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
                RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)
                "
            )),
            2
        );
        assert_eq!(
            part1(indoc!(
                "
                LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)
                "
            )),
            6
        );
    }
}
