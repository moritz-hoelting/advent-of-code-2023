use std::{cmp::Ordering, collections::HashMap};

const REPETITIONS: u32 = 1_000_000_000;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
    let mut lines = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut seen = HashMap::new();
    seen.insert(lines.clone(), 0);

    let mut stop_at = None;

    for i in 1..REPETITIONS + 1 {
        lines = cycle(lines);
        // stop at correct iteration
        if stop_at.is_some_and(|j| j == i) {
            break;
        }
        // check if we've seen this before & calculate where to stop
        if seen.contains_key(&lines) && stop_at.is_none() {
            let repeat_period = i - seen.get(&lines).unwrap();
            stop_at = Some(i + ((REPETITIONS - i) % repeat_period));
        }
        seen.insert(lines.clone(), i);
    }

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == 'O').count() * (lines.len() - i))
        .sum()
}

fn cycle(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north_lines = transmute(slide_rows_left(transmute(matrix)));

    let west_lines = slide_rows_left(north_lines);

    let south_lines = transmute(reverse_rows(slide_rows_left(reverse_rows(transmute(
        west_lines,
    )))));

    reverse_rows(slide_rows_left(reverse_rows(south_lines)))
}

fn transmute<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i| {
            matrix
                .iter()
                .map(|row| *row.get(i).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn reverse_rows<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix
        .iter()
        .map(|row| {
            let mut row = row.clone();
            row.reverse();
            row
        })
        .collect::<Vec<_>>()
}

fn slide_rows_left(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|mut row| {
            for n in (1..row.len()).rev() {
                for i in 0..n {
                    if sort_slide_line(&row[i], &row[i + 1]) == Ordering::Greater {
                        row.swap(i, i + 1);
                    }
                }
            }

            row
        })
        .collect()
}

fn sort_slide_line(a: &char, b: &char) -> Ordering {
    if a == &'#' || b == &'#' {
        Ordering::Equal
    } else if a == &'.' && b == &'O' {
        Ordering::Greater
    } else if a == &'O' && b == &'.' {
        Ordering::Less
    } else {
        a.cmp(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc!(
        "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        "
    );

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 64);
    }

    #[test]
    fn test_cycle() {
        let lines = INPUT
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();
        // one cycle
        assert_eq!(
            cycle(lines.clone()),
            indoc!(
                "
                .....#....
                ....#...O#
                ...OO##...
                .OO#......
                .....OOO#.
                .O#...O#.#
                ....O#....
                ......OOOO
                #...O###..
                #..OO#...."
            )
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>()
        );
        // two cycles
        assert_eq!(
            cycle(cycle(lines.clone())),
            indoc!(
                "
                .....#....
                ....#...O#
                .....##...
                ..O#......
                .....OOO#.
                .O#...O#.#
                ....O#...O
                .......OOO
                #..OO###..
                #.OOO#...O"
            )
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>()
        );
        // three cycles
        assert_eq!(
            cycle(cycle(cycle(lines.clone()))),
            indoc!(
                "
                .....#....
                ....#...O#
                .....##...
                ..O#......
                .....OOO#.
                .O#...O#.#
                ....O#...O
                .......OOO
                #...O###.O
                #.OOO#...O"
            )
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>()
        );
    }
}
