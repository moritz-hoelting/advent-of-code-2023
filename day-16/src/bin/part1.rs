use std::{collections::HashSet, iter};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile::try_from(c).expect("invalid char"))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut energized = HashSet::new();

    let start = vec![(0, 0, Direction::Right)];

    let _ = iter::successors(Some(start), |curs| {
        let next = curs
            .iter()
            .flat_map(|cur| {
                if energized.contains(cur) {
                    Vec::new()
                } else {
                    energized.insert(*cur);
                    successors(*cur, &grid)
                }
            })
            .collect::<Vec<_>>();
        (!next.is_empty()).then_some(next)
    })
    .collect::<Vec<_>>();

    energized
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>()
        .len()
}

fn successors(
    (x, y, direction): (usize, usize, Direction),
    grid: &Vec<Vec<Tile>>,
) -> Vec<(usize, usize, Direction)> {
    let prev = grid[y][x];
    let above = y.checked_sub(1).map(|y| (x, y));
    let below = (y + 1 < grid.len()).then_some((x, y + 1));
    let left = x.checked_sub(1).map(|x| (x, y));
    let right = (x + 1 < grid[0].len()).then_some((x + 1, y));

    match prev {
        Tile::Empty => match direction {
            Direction::Up => vec![above.map(|(x, y)| (x, y, Direction::Up))],
            Direction::Down => vec![below.map(|(x, y)| (x, y, Direction::Down))],
            Direction::Left => vec![left.map(|(x, y)| (x, y, Direction::Left))],
            Direction::Right => vec![right.map(|(x, y)| (x, y, Direction::Right))],
        },
        Tile::MirrorForward => match direction {
            Direction::Up => vec![right.map(|(x, y)| (x, y, Direction::Right))],
            Direction::Down => vec![left.map(|(x, y)| (x, y, Direction::Left))],
            Direction::Left => vec![below.map(|(x, y)| (x, y, Direction::Down))],
            Direction::Right => vec![above.map(|(x, y)| (x, y, Direction::Up))],
        },
        Tile::MirrorBackward => match direction {
            Direction::Up => vec![left.map(|(x, y)| (x, y, Direction::Left))],
            Direction::Down => vec![right.map(|(x, y)| (x, y, Direction::Right))],
            Direction::Left => vec![above.map(|(x, y)| (x, y, Direction::Up))],
            Direction::Right => vec![below.map(|(x, y)| (x, y, Direction::Down))],
        },
        Tile::SplitHorizontal => match direction {
            Direction::Up | Direction::Down => vec![
                left.map(|(x, y)| (x, y, Direction::Left)),
                right.map(|(x, y)| (x, y, Direction::Right)),
            ],
            Direction::Left => vec![left.map(|(x, y)| (x, y, Direction::Left))],
            Direction::Right => vec![right.map(|(x, y)| (x, y, Direction::Right))],
        },
        Tile::SplitVertical => match direction {
            Direction::Left | Direction::Right => vec![
                above.map(|(x, y)| (x, y, Direction::Up)),
                below.map(|(x, y)| (x, y, Direction::Down)),
            ],
            Direction::Up => vec![above.map(|(x, y)| (x, y, Direction::Up))],
            Direction::Down => vec![below.map(|(x, y)| (x, y, Direction::Down))],
        },
    }
    .into_iter()
    .flatten()
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    SplitHorizontal,
    SplitVertical,
    MirrorForward,
    MirrorBackward,
}
impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '|' => Ok(Self::SplitVertical),
            '-' => Ok(Self::SplitHorizontal),
            '/' => Ok(Self::MirrorForward),
            '\\' => Ok(Self::MirrorBackward),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
                r"
                .|...\....
                |.-.\.....
                .....|-...
                ........|.
                ..........
                .........\
                ..../.\\..
                .-.-/..|..
                .|....-|.\
                ..//.|....
                "
            )),
            46
        );
    }
}
