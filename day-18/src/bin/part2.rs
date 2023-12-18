fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| Instruction::try_from(line).expect("invalid instruction"))
        .collect::<Vec<_>>();

    let (corners, length) =
        instructions
            .iter()
            .fold((vec![(0, 0)], 0), |(mut corners, length), instruction| {
                let last_corner = corners.last().unwrap();
                let new_corner = match instruction.direction {
                    Direction::Up => (last_corner.0, last_corner.1 - instruction.distance),
                    Direction::Down => (last_corner.0, last_corner.1 + instruction.distance),
                    Direction::Left => (last_corner.0 - instruction.distance, last_corner.1),
                    Direction::Right => (last_corner.0 + instruction.distance, last_corner.1),
                };
                corners.push(new_corner);
                (corners, length + instruction.distance)
            });

    // shoelace formula with edges also du out
    corners
        .iter()
        .zip(&corners[1..])
        .map(|(v1, v2)| (v1.0 - v2.0) * (v1.1 + v2.1))
        .sum::<i64>()
        .abs()
        / 2
        + length / 2
        + 1
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: i64,
}
impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();
        let hex = parts
            .nth(2)
            .expect("no color")
            .trim_start_matches("(#")
            .trim_end_matches(')');
        let distance = i64::from_str_radix(&hex[0..5], 16).map_err(|_| "invalid distance")?;
        let direction = match hex.chars().nth(5).ok_or("no direction")? {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => return Err("invalid direction"),
        };

        Ok(Self {
            direction,
            distance,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("invalid direction"),
        }
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
                R 6 (#70c710)
                D 5 (#0dc571)
                L 2 (#5713f0)
                D 2 (#d2c081)
                R 2 (#59c680)
                D 2 (#411b91)
                L 5 (#8ceee2)
                U 2 (#caa173)
                L 1 (#1b58a2)
                U 2 (#caa171)
                R 2 (#7807d2)
                U 3 (#a77fa3)
                L 2 (#015232)
                U 2 (#7a21e3)
                "
            )),
            952408144115
        );
    }
}
