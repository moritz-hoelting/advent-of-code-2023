fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> i32 {
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
        .sum::<i32>()
        .abs()
        / 2
        + length / 2
        + 1
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: i32,
    color: u32,
}
impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();
        let direction =
            Direction::try_from(parts.next().expect("no direction").chars().next().unwrap())
                .expect("invalid direction");
        let distance = parts
            .next()
            .expect("no distance")
            .parse::<i32>()
            .expect("invalid distance");
        let color = u32::from_str_radix(
            parts
                .next()
                .expect("no color")
                .trim_start_matches("(#")
                .trim_end_matches(')'),
            16,
        )
        .expect("invalid color");
        Ok(Self {
            direction,
            distance,
            color,
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
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
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
            62
        );
    }
}
