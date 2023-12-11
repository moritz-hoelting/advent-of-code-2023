use indicatif::ParallelProgressIterator;
use pathfinding::prelude::bfs;
use rayon::prelude::*;

fn main() {
    println!("{}", part2(include_str!("./input.txt"), 1_000_000));
}

fn part2(input: &str, factor: usize) -> usize {
    let mut lines = input
        .lines()
        .map(|l| {
            let row = l
                .chars()
                .map(|c| Space::try_from(c).expect("invalid char"))
                .collect::<Vec<_>>();
            if row.iter().all(|s| *s == Space::Empty) {
                row.into_par_iter()
                    .map(|_| Space::ExpandedEmpty)
                    .collect::<Vec<_>>()
            } else {
                row
            }
        })
        .collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    for i in (0..width).rev() {
        if (0..height).all(|j| {
            matches!(
                lines.get(j).unwrap().get(i).unwrap(),
                Space::Empty | Space::ExpandedEmpty
            )
        }) {
            (0..height).for_each(|j| {
                if let Some(s) = lines[j].get_mut(i) {
                    *s = Space::ExpandedEmpty;
                }
            });
        }
    }

    let galaxies = lines
        .par_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.par_iter()
                .enumerate()
                .filter_map(move |(x, s)| (s == &Space::Galaxy).then_some((x, y)))
        })
        .collect::<Vec<_>>();

    let pairs = galaxies
        .par_iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            galaxies
                .par_iter()
                .enumerate()
                .filter_map(move |(j, (x2, y2))| {
                    if j > i {
                        Some(((*x1, *y1), (*x2, *y2)))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let distances = pairs
        .into_par_iter()
        .progress()
        .filter_map(|(start, target)| {
            bfs(
                &start,
                |coords| successors(coords, &target, width, height),
                |cur| *cur == target,
            )
            .map(|path| {
                path.iter()
                    .map(|(x, y)| {
                        let space = lines.get(*y).unwrap().get(*x).unwrap();
                        if matches!(space, Space::ExpandedEmpty) {
                            factor
                        } else {
                            1
                        }
                    })
                    .sum::<usize>()
                    - 1
            })
        })
        .collect::<Vec<_>>();

    distances.iter().sum()
}

fn successors(
    (x, y): &(usize, usize),
    (tx, ty): &(usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if tx < x && *x > 0 {
        neighbors.push((*x - 1, *y));
    }
    if tx > x && *x < width - 1 {
        neighbors.push((*x + 1, *y));
    }
    if ty < y && *y > 0 {
        neighbors.push((*x, *y - 1));
    }
    if ty > y && *y < height - 1 {
        neighbors.push((*x, *y + 1));
    }
    neighbors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    ExpandedEmpty,
    Galaxy,
}
impl TryFrom<char> for Space {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Galaxy),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part2() {
        const INPUT: &str = indoc!(
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        );
        assert_eq!(part2(INPUT, 10), 1030);
        assert_eq!(part2(INPUT, 100), 8410);
    }
}
