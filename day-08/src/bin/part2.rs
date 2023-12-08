use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::{is_a, tag, take_while1},
    character::{
        complete::{char, line_ending},
        is_alphanumeric,
    },
    combinator::map,
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use num::integer;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
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

    let starting_nodes = nodes
        .iter()
        .filter(|(n, (_, _))| n.ends_with('A'))
        .map(|(n, (_, _))| *n)
        .collect::<Vec<_>>();

    starting_nodes
        .into_iter()
        .map(|n| find_steps_to_goal(n, &nodes, &directions))
        .fold(1_usize, integer::lcm)
}

fn find_steps_to_goal(
    starting_node: &str,
    nodes: &HashMap<&str, (&str, &str)>,
    directions: &[Direction],
) -> usize {
    match directions.iter().cycle().enumerate().fold_while(
        (starting_node, 0),
        |(acc, _), (i, d)| {
            let (l, r) = nodes.get(acc).unwrap();
            let next = match d {
                Direction::Right => r,
                Direction::Left => l,
            };

            if next.ends_with('Z') {
                FoldWhile::Done((next, i))
            } else {
                FoldWhile::Continue((next, i))
            }
        },
    ) {
        FoldWhile::Done((_, i)) => i + 1,
        FoldWhile::Continue(_) => panic!("invalid input"),
    }
}

fn node_parser(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        take_while1(|c| is_alphanumeric(c as u8)),
        tag(" = "),
        delimited(
            char('('),
            separated_pair(
                take_while1(|c| is_alphanumeric(c as u8)),
                tag(", "),
                take_while1(|c| is_alphanumeric(c as u8)),
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
    fn test_part2() {
        assert_eq!(
            part2(indoc!(
                "
                LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)
                "
            )),
            6
        );
    }
}
