use itertools::Itertools;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> i64 {
    input.lines().map(line).sum()
}

fn line(input: &str) -> i64 {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("invalid digit"))
        .collect::<Vec<_>>();

    find_previous(&nums)
}

fn find_previous(line: &[i64]) -> i64 {
    let differences = line
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    let first = *line.first().expect("invalid input");

    if differences.iter().all(|d| *d == 0) {
        first
    } else {
        first - find_previous(&differences)
    }
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
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            "
            )),
            2
        );
    }

    #[test]
    fn test_lines_part2() {
        assert_eq!(line("0 3 6 9 12 15"), -3);
        assert_eq!(line("1 3 6 10 15 21"), 0);
        assert_eq!(line("10  13  16  21  30  45"), 5);
    }
}
