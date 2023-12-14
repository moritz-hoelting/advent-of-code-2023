use std::cmp::Ordering;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let slid_lines = slide_north(input);

    slid_lines
        .iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == 'O').count() * (slid_lines.len() - i))
        .sum()
}

fn slide_north(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<_>>();
    let columns = (0..lines[0].len())
        .map(|i| {
            let mut column = lines
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<Vec<_>>();

            // bubble sort
            for n in (1..column.len()).rev() {
                for j in 0..n {
                    if sort_slide_north(&column[j], &column[j + 1]) == Ordering::Greater {
                        column.swap(j, j + 1);
                    }
                }
            }

            column
        })
        .collect::<Vec<_>>();

    (0..columns[0].len())
        .map(|i| {
            columns
                .iter()
                .map(|column| *column.get(i).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sort_slide_north(a: &char, b: &char) -> Ordering {
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
    fn test_part1() {
        assert_eq!(part1(INPUT), 136);
    }

    #[test]
    fn test_slide_north() {
        assert_eq!(
            slide_north(INPUT)
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
            indoc!(
                "
                OOOO.#.O..
                OO..#....#
                OO..O##..O
                O..#.OO...
                ........#.
                ..#....#.#
                ..O..#.O.O
                ..O.......
                #....###..
                #....#...."
            )
        );
    }
}
