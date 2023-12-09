use itertools::Itertools;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> i64 {
    input.lines().map(line).sum()
}

fn line(input: &str) -> i64 {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("invalid digit"))
        .collect::<Vec<_>>();

    find_next(&nums)
}

fn find_next(line: &[i64]) -> i64 {
    let differences = line
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    let last = *line.last().expect("invalid input");

    if differences.iter().all(|d| *d == 0) {
        last
    } else {
        last + find_next(&differences)
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
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            "
            )),
            114
        );
    }

    #[test]
    fn test_lines_part1() {
        assert_eq!(line("0 3 6 9 12 15"), 18);
        assert_eq!(line("1 3 6 10 15 21"), 28);
        assert_eq!(line("10  13  16  21  30  45"), 68);
    }
}
