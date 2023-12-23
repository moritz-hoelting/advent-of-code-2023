use std::collections::{HashSet, VecDeque};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<_>>>();

    let start = map
        .get(0)
        .and_then(|row| row.iter().position(|tile| *tile == Tile::Path))
        .map(|x| (x, 0_usize))
        .expect("No start found");

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut max_steps = Vec::new();

    let mut queue = VecDeque::from(vec![(start, visited, 0)]);

    while let Some((coord, visited, steps)) = queue.pop_front() {
        let next = successors(coord, &map, &visited);

        if next.is_empty() {
            max_steps.push(steps);
        } else {
            queue.extend(
                next.into_iter()
                    .map(|(coord, visited)| (coord, visited, steps + 1)),
            );
        }
    }

    max_steps.into_iter().max().unwrap_or_default()
}

type Coord = (usize, usize);

fn successors(
    (x, y): Coord,
    map: &[Vec<Tile>],
    visited: &HashSet<(usize, usize)>,
) -> Vec<(Coord, HashSet<Coord>)> {
    let above = y
        .checked_sub(1)
        .and_then(|yn| map.get(yn))
        .and_then(|row| row.get(x));
    let below = map.get(y + 1).and_then(|row| row.get(x));
    let left = x
        .checked_sub(1)
        .and_then(|xn| map.get(y).and_then(|row| row.get(xn)));
    let right = map.get(y).and_then(|row| row.get(x + 1));

    let coords = match map
        .get(y)
        .and_then(|row| row.get(x))
        .expect("Invalid position")
    {
        Tile::Forest => panic!("Forest"),
        Tile::Path => {
            let mut next = Vec::new();
            match above {
                Some(Tile::Forest) | None => {}
                Some(_) => next.push((x, y - 1)),
            }
            match right {
                Some(Tile::Forest) | None => {}
                Some(_) => next.push((x + 1, y)),
            }
            match below {
                Some(Tile::Forest) | None => {}
                Some(_) => next.push((x, y + 1)),
            }
            match left {
                Some(Tile::Forest) | None => {}
                Some(_) => next.push((x - 1, y)),
            }
            next
        }
        Tile::SlopeUp => above
            .is_some_and(|t| t != &Tile::Forest)
            .then_some(vec![(x, y - 1)])
            .unwrap_or_default(),
        Tile::SlopeRight => right
            .is_some_and(|t| t != &Tile::Forest)
            .then_some(vec![(x + 1, y)])
            .unwrap_or_default(),
        Tile::SlopeDown => below
            .is_some_and(|t| t != &Tile::Forest)
            .then_some(vec![(x, y + 1)])
            .unwrap_or_default(),
        Tile::SlopeLeft => left
            .is_some_and(|t| t != &Tile::Forest)
            .then_some(vec![(x - 1, y)])
            .unwrap_or_default(),
    };

    coords
        .into_iter()
        .filter(|coord| !visited.contains(coord))
        .map(|coord| {
            let mut visited = visited.clone();
            visited.insert(coord);
            (coord, visited)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' => Self::Path,
            '>' => Self::SlopeRight,
            '<' => Self::SlopeLeft,
            '^' => Self::SlopeUp,
            'v' => Self::SlopeDown,
            _ => panic!("Invalid tile: {}", value),
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
                "
                #.#####################
                #.......#########...###
                #######.#########.#.###
                ###.....#.>.>.###.#.###
                ###v#####.#v#.###.#.###
                ###.>...#.#.#.....#...#
                ###v###.#.#.#########.#
                ###...#.#.#.......#...#
                #####.#.#.#######.#.###
                #.....#.#.#.......#...#
                #.#####.#.#.#########v#
                #.#...#...#...###...>.#
                #.#.#v#######v###.###v#
                #...#.>.#...>.>.#.###.#
                #####v#.#.###v#.#.###.#
                #.....#...#...#.#.#...#
                #.#########.###.#.#.###
                #...###...#...#...#.###
                ###.###.#.###v#####v###
                #...#...#.#.>.>.#.>.###
                #.###.###.#.###.#.#v###
                #.....###...###...#...#
                #####################.#
                "
            )),
            94
        );
    }
}
