use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("./input.txt"), 64));
}

fn part1(input: &str, steps: u32) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().map(Plot::from).collect())
        .collect::<Vec<Vec<_>>>();

    let starting = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, p)| {
                if p == &Plot::Starting {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .expect("no starting plot");

    let reachable = std::iter::successors(Some(vec![(starting, 0)]), |prev| {
        let next = prev
            .iter()
            .flat_map(|p| {
                successors(*p, &map)
                    .into_iter()
                    .filter(|(_, cur_steps)| *cur_steps <= steps)
            })
            .sorted()
            .dedup()
            .collect::<Vec<_>>();
        if next.is_empty() {
            None
        } else {
            Some(next)
        }
    })
    .collect::<Vec<_>>();

    reachable.last().expect("no last element").len()
}

fn successors(
    ((x, y), step_count): ((usize, usize), u32),
    map: &Vec<Vec<Plot>>,
) -> Vec<((usize, usize), u32)> {
    let mut successors = vec![];
    if x > 0 && map[y][x - 1] != Plot::Stone {
        successors.push(((x - 1, y), step_count + 1));
    }
    if y > 0 && map[y - 1][x] != Plot::Stone {
        successors.push(((x, y - 1), step_count + 1));
    }
    if x < map[0].len() - 1 && map[y][x + 1] != Plot::Stone {
        successors.push(((x + 1, y), step_count + 1));
    }
    if y < map.len() - 1 && map[y + 1][x] != Plot::Stone {
        successors.push(((x, y + 1), step_count + 1));
    }
    successors
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Plot {
    Starting,
    Garden,
    Stone,
}
impl From<char> for Plot {
    fn from(c: char) -> Self {
        match c {
            '.' => Plot::Garden,
            '#' => Plot::Stone,
            'S' => Plot::Starting,
            _ => panic!("Invalid plot"),
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
            part1(
                indoc!(
                    "
                ...........
                .....###.#.
                .###.##..#.
                ..#.#...#..
                ....#.#....
                .##..S####.
                .##..#...#.
                .......##..
                .##.#.####.
                .##..##.##.
                ...........
                "
                ),
                6
            ),
            16
        );
    }
}
