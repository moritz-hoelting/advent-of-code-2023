use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let mut bricks = input
        .lines()
        .map(|line| {
            let mut ends = line.split('~').map(|part| {
                let mut coords = part.splitn(3, ',');
                let x = coords.next().unwrap().parse::<u16>().unwrap();
                let y = coords.next().unwrap().parse::<u16>().unwrap();
                let z = coords.next().unwrap().parse::<u16>().unwrap();

                (x, y, z)
            });
            let start = ends.next().unwrap();
            let end = ends.next().unwrap();

            Brick::new(start, end)
        })
        .sorted_by_key(|brick| brick.z.0)
        .collect::<Vec<_>>();

    // simulate bricks falling
    for index in 0..bricks.len() {
        let mut max_z = 1;
        for check in &bricks[..index] {
            if check.overlaps(bricks.get(index).unwrap()) {
                max_z = max_z.max(check.z.1 + 1);
            }
        }
        let brick = bricks.get_mut(index).unwrap();
        brick.z.1 -= brick.z.0 - max_z;
        brick.z.0 = max_z;
    }
    bricks.sort_by_key(|brick| brick.z.0);

    let mut supports = (0..bricks.len())
        .map(|i| (i, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut supported_by = (0..bricks.len())
        .map(|i| (i, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();

    // find all bricks that support other bricks
    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if upper.overlaps(lower) && upper.z.0 == lower.z.1 + 1 {
                supports.get_mut(&i).unwrap().insert(j);
                supported_by.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    // count all the bricks for which all the bricks that
    // it supports have more than one supporter
    supports
        .values()
        .filter(|supported| {
            supported
                .iter()
                .all(|brick| supported_by.get(brick).unwrap().len() > 1)
        })
        .count()
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    x: (u16, u16),
    y: (u16, u16),
    z: (u16, u16),
}
impl Brick {
    fn new(start: (u16, u16, u16), end: (u16, u16, u16)) -> Self {
        Self {
            x: (start.0, end.0),
            y: (start.1, end.1),
            z: (start.2, end.2),
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x.1 >= other.x.0
            && other.x.1 >= self.x.0
            && self.y.1 >= other.y.0
            && other.y.1 >= self.y.0
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
                1,0,1~1,2,1
                0,0,2~2,0,2
                0,2,3~2,2,3
                0,0,4~0,2,4
                2,0,5~2,2,5
                0,1,6~2,1,6
                1,1,8~1,1,9
                "
            )),
            5
        );
    }

    #[test]
    fn test_overlap() {
        let brick_a = Brick::new((0, 0, 0), (2, 1, 1));
        let brick_b = Brick::new((1, 0, 0), (1, 1, 1));

        assert!(brick_a.overlaps(&brick_b));
    }
}
