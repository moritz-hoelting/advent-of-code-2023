fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

fn process_pattern(pattern: &str) -> usize {
    let lines = pattern
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut sum = check_for_reflection(lines.clone()) * 100;
    let width = lines[0].len();
    let columns = (0..width)
        .map(|i| lines.iter().map(|row| row[i]).collect::<Vec<char>>())
        .collect();
    sum += check_for_reflection(columns);

    sum
}

fn check_for_reflection(pattern: Vec<Vec<char>>) -> usize {
    (1..pattern.len())
        .find(|i| {
            let a = &pattern[0..*i];
            let b = &pattern[*i..];

            a.iter().rev().zip(b).all(|(a, b)| a == b)
        })
        .unwrap_or(0)
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
            405
        );
    }
}
