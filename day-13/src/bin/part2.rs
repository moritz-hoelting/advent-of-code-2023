fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

fn process_pattern(pattern: &str) -> usize {
    let lines = pattern
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let width = lines[0].len();
    (0..lines.len())
        .find_map(move |i| {
            let lines = lines.clone();
            (0..width).find_map(move |j| {
                let mut local_lines = lines
                    .clone()
                    .iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<Vec<_>>>();
                let smudge = local_lines.get_mut(i).unwrap().get_mut(j).unwrap();
                *smudge = if smudge == &'#' { '.' } else { '#' };

                let mut sum = check_for_reflection(local_lines.clone(), i) * 100;
                if sum > 0 {
                    return Some(sum);
                }

                let columns = (0..width)
                    .map(|i| local_lines.iter().map(|row| row[i]).collect::<Vec<char>>())
                    .collect();
                sum += check_for_reflection(columns, j);

                if sum > 0 {
                    Some(sum)
                } else {
                    None
                }
            })
        })
        .expect("no reflection found")
}

fn check_for_reflection(pattern: Vec<Vec<char>>, must_include: usize) -> usize {
    (1..pattern.len())
        .find(|i| {
            let a = &pattern[0..*i];
            let b = &pattern[*i..];

            let diff = must_include.abs_diff(*i);

            let zipped = a.iter().rev().zip(b).collect::<Vec<_>>();

            zipped.len() > diff && zipped.iter().all(|(a, b)| a == b)
        })
        .unwrap_or(0)
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
                #.##..##.
                ..#.##.#.
                ##......#
                ##......#
                ..#.##.#.
                ..##..##.
                #.#.##.#.

                #...##..#
                #....#..#
                ..##..###
                #####.##.
                #####.##.
                ..##..###
                #....#..#
                "
            )),
            400
        );
    }

    #[test]
    fn test_pattern_part2() {
        assert_eq!(
            process_pattern(indoc!(
                "
                #.##..##.
                ..#.##.#.
                ##......#
                ##......#
                ..#.##.#.
                ..##..##.
                #.#.##.#.
                "
            )),
            300
        );

        assert_eq!(
            process_pattern(indoc!(
                "
                #...##..#
                #....#..#
                ..##..###
                #####.##.
                #####.##.
                ..##..###
                #....#..#
                "
            )),
            100
        );
    }
}
