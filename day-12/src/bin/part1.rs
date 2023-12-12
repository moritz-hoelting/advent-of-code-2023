use nom::{branch::alt, character::complete, combinator::value, multi::many1};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let (springs_str, group_size_str) = line.split_once(' ').expect("Invalid input");

    let group_size = group_size_str
        .split(',')
        .map(|s| s.parse::<usize>().expect("invalid char as number"))
        .collect::<Vec<_>>();

    let springs = many1(alt((
        value(
            SpringStatus::Operational,
            complete::char::<&str, nom::error::Error<&str>>('.'),
        ),
        value(
            SpringStatus::Damaged,
            complete::char::<&str, nom::error::Error<&str>>('#'),
        ),
        value(
            SpringStatus::Unknown,
            complete::char::<&str, nom::error::Error<&str>>('?'),
        ),
    )))(springs_str)
    .expect("Invalid input")
    .1;

    let spring_amount = springs.len();
    let groups_amount = group_size.len();
    let mut dp = vec![vec![vec![0; spring_amount + 1]; groups_amount + 1]; spring_amount + 1];

    dp[spring_amount][groups_amount][0] = 1;
    dp[spring_amount][groups_amount - 1][group_size[groups_amount - 1]] = 1;

    for pos in (0..spring_amount).rev() {
        for (group, &max_count) in group_size.iter().enumerate() {
            // try iteratively all possible counts for the current group
            for count in 0..=max_count {
                // try both operational and damaged for each position
                for &c in &[SpringStatus::Operational, SpringStatus::Damaged] {
                    // only proceed if the spring is of the chosen type or unknown
                    if springs[pos] == c || springs[pos] == SpringStatus::Unknown {
                        if c == SpringStatus::Operational && count == 0 {
                            // if operational and count is 0, then add the value from the next position because
                            // there is no new combination
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == SpringStatus::Operational && group_size[group] == count {
                            // if operational and count is equal to the group size, then add the value from the
                            // next position and next group
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == SpringStatus::Damaged {
                            // if damaged, then add the value from the next position and next count
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(
            springs[pos],
            SpringStatus::Operational | SpringStatus::Unknown
        ) {
            dp[pos][groups_amount][0] += dp[pos + 1][groups_amount][0];
        }
    }

    dp[0][0][0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
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
                ???.### 1,1,3
                .??..??...?##. 1,1,3
                ?#?#?#?#?#?#?#? 1,3,1,6
                ????.#...#... 4,1,1
                ????.######..#####. 1,6,5
                ?###???????? 3,2,1
                "
            )),
            21
        );
    }

    #[test]
    fn test_process_line_part1() {
        assert_eq!(process_line("???.### 1,1,3"), 1);
        assert_eq!(process_line(".??..??...?##. 1,1,3"), 4);
        assert_eq!(process_line("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(process_line("????.#...#... 4,1,1"), 1);
        assert_eq!(process_line("????.######..#####. 1,6,5"), 4);
        assert_eq!(process_line("?###???????? 3,2,1"), 10);
    }
}
