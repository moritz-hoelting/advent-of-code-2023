use indicatif::ParallelProgressIterator;
use pathfinding::prelude::bfs;
use rayon::prelude::*;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let mut lines = input
        .lines()
        .flat_map(|l| {
            let row = l
                .chars()
                .map(|c| Space::try_from(c).expect("invalid char"))
                .collect::<Vec<_>>();
            if row.iter().all(|s| *s == Space::Empty) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .collect::<Vec<_>>();

    let height = lines.len();
    let width = lines[0].len();

    for i in (0..width).rev() {
        if (0..height).all(|j| lines.get(j).unwrap().get(i).unwrap() == &Space::Empty) {
            (0..height).for_each(|j| {
                lines[j].insert(i, Space::Empty);
            });
        }
    }

    let height = lines.len();
    let width = lines[0].len();

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
            .map(|path| path.len() - 1)
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
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
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
            )),
            374
        );
    }
}
