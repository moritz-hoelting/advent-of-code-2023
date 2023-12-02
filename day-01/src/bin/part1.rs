fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    input.lines().map(line).sum()
}

fn line(input: &str) -> u32 {
    let mut numbers = input.chars().filter_map(|c| c.to_digit(10)).peekable();
    let first = numbers.peek().unwrap().to_owned();
    let last = numbers.last().unwrap();
    (first * 10) + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
    }

    #[test]
    fn test_line1() {
        assert_eq!(line("1abc2"), 12);
        assert_eq!(line("pqr3stu8vwx"), 38);
        assert_eq!(line("a1b2c3d4e5f"), 15);
        assert_eq!(line("treb7uchet"), 77);
    }
}
