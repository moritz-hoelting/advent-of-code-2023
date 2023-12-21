use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("{}", part2(include_str!("./input.txt"), 26501365));
}

/// Assumptions for part 2:
/// Line and column of the starting point are completely empty
/// Borders are completely empty
/// The garden is of odd size with the starting point in the middle
/// The garden is a square
/// The step count is a whole number of the gardens width plus half the width
fn part2(input: &str, steps: u32) -> usize {
    /* IDEA:
    the inner repetitions of the garden have all the same amount of reachable plots,
    grouped by whether they are entered with an odd or even amount of steps.
    The corners of this diagonal square of reachable plots have the size of the
    garden minus one steps remaining and are different from each other.
    the small triangles have half the size of the garden minus one steps remaining
    and are entered from the corners.
    the big triangles have the size of the garden plus half the size of the garden
    of steps remaining and are also entered from the corners.
    */

    let map = input
        .lines()
        .map(|line| line.chars().map(Plot::from).collect())
        .collect::<Vec<Vec<_>>>();

    assert!(map.len() == map[0].len(), "garden is not a square");
    assert!(
        steps as usize % map.len() == map.len() / 2,
        "steps is not a whole number of the gardens width plus half the width"
    );

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

    assert!(
        starting.0 == map.len() / 2 && starting.1 == map.len() / 2,
        "starting point is not in the middle of the garden"
    );

    let grid_width = steps as usize / map.len() - 1;

    // round down to even number
    let odd_grids_amount = (grid_width / 2 * 2 + 1).pow(2);
    let even_grids_amount = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points_each = fill(starting, map.len() as u32 * 2 + 1, &map);
    let even_points_each = fill(starting, map.len() as u32 * 2, &map);

    let top_corner_points = fill((starting.0, map.len() - 1), map.len() as u32 - 1, &map);
    let right_corner_points = fill((0, starting.1), map.len() as u32 - 1, &map);
    let bottom_corner_points = fill((starting.0, 0), map.len() as u32 - 1, &map);
    let left_corner_points = fill((map.len() - 1, starting.1), map.len() as u32 - 1, &map);

    let corner_points =
        top_corner_points + right_corner_points + bottom_corner_points + left_corner_points;

    let small_top_right_points = fill((0, map.len() - 1), map.len() as u32 / 2 - 1, &map);
    let small_top_left_points = fill(
        (map.len() - 1, map.len() - 1),
        map.len() as u32 / 2 - 1,
        &map,
    );
    let small_bottom_right_points = fill((0, 0), map.len() as u32 / 2 - 1, &map);
    let small_bottom_left_points = fill((map.len() - 1, 0), map.len() as u32 / 2 - 1, &map);

    let small_points = (small_top_right_points
        + small_top_left_points
        + small_bottom_right_points
        + small_bottom_left_points)
        * (grid_width + 1);

    let large_top_right_points = fill((0, map.len() - 1), map.len() as u32 * 3 / 2 - 1, &map);
    let large_top_left_points = fill(
        (map.len() - 1, map.len() - 1),
        map.len() as u32 * 3 / 2 - 1,
        &map,
    );
    let large_bottom_right_points = fill((0, 0), map.len() as u32 * 3 / 2 - 1, &map);
    let large_bottom_left_points = fill((map.len() - 1, 0), map.len() as u32 * 3 / 2 - 1, &map);

    let large_points = (large_top_right_points
        + large_top_left_points
        + large_bottom_right_points
        + large_bottom_left_points)
        * grid_width;

    odd_grids_amount * odd_points_each
        + even_grids_amount * even_points_each
        + corner_points
        + small_points
        + large_points
}

fn fill((starting_x, starting_y): (usize, usize), steps: u32, map: &Vec<Vec<Plot>>) -> usize {
    let mut ans = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert((starting_x, starting_y));
    let mut queue = VecDeque::new();
    queue.push_back(((starting_x, starting_y), steps));

    while let Some(((x, y), remaining_steps)) = queue.pop_front() {
        if remaining_steps % 2 == 0 {
            ans.insert((x, y));
        }
        if remaining_steps == 0 {
            continue;
        }

        let directions = [
            Some((x + 1, y)),
            x.checked_sub(1).map(|x| (x, y)),
            Some((x, y + 1)),
            y.checked_sub(1).map(|y| (x, y)),
        ];

        for (nx, ny) in directions.into_iter().flatten() {
            if ny >= map.len()
                || nx >= map[0].len()
                || seen.contains(&(nx, ny))
                || map[ny][nx] == Plot::Stone
            {
                continue;
            }
            seen.insert((nx, ny));
            queue.push_back(((nx, ny), remaining_steps - 1))
        }
    }

    ans.len()
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

#[allow(dead_code)]
/// Bruteforce
/// Takes too long
fn part2_bruteforce(input: &str, steps: u32) -> usize {
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
                    Some((x as i128, y as i128))
                } else {
                    None
                }
            })
        })
        .expect("no starting plot");

    let mut visited = HashSet::new();
    visited.insert(starting);
    let mut new = HashSet::new();
    new.insert(starting);
    let mut cache = HashMap::new();
    cache.insert(0, 1);

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
    ((x, y), step_count): ((i128, i128), u32),
    map: &Vec<Vec<Plot>>,
) -> Vec<((i128, i128), u32)> {
    let height = map.len();
    let width = map[0].len();
    let ux = (x.rem_euclid(width as i128)) as usize;
    let uy = (y.rem_euclid(height as i128)) as usize;
    let mut successors = vec![];
    if (ux > 0 && map[uy][ux - 1] != Plot::Stone) || (ux == 0 && map[uy][width - 1] != Plot::Stone)
    {
        successors.push(((x - 1, y), step_count + 1));
    }
    if (uy > 0 && map[uy - 1][ux] != Plot::Stone) || (uy == 0 && map[height - 1][ux] != Plot::Stone)
    {
        successors.push(((x, y - 1), step_count + 1));
    }
    if (ux < width - 1 && map[uy][ux + 1] != Plot::Stone)
        || (ux == width - 1 && map[uy][0] != Plot::Stone)
    {
        successors.push(((x + 1, y), step_count + 1));
    }
    if (uy < height - 1 && map[uy + 1][ux] != Plot::Stone)
        || (uy == height - 1 && map[0][ux] != Plot::Stone)
    {
        successors.push(((x, y + 1), step_count + 1));
    }
    successors
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc!(
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
    );

    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_6() {
        assert_eq!(part2(INPUT, 6), 16);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_10() {
        assert_eq!(part2(INPUT, 10), 50);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_50() {
        assert_eq!(part2(INPUT, 50), 1594);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_100() {
        assert_eq!(part2(INPUT, 100), 6536);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_500() {
        assert_eq!(part2(INPUT, 500), 167004);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_1000() {
        assert_eq!(part2(INPUT, 1000), 668697);
    }
    #[test]
    #[ignore = "not working because of assumptions"]
    fn test_part2_5000() {
        assert_eq!(part2(INPUT, 5000), 16733044);
    }
}
