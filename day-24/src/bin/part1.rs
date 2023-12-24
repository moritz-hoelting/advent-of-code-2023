use itertools::Itertools;

fn main() {
    println!(
        "{}",
        part1(
            include_str!("./input.txt"),
            200_000_000_000_000,
            400_000_000_000_000
        )
    );
}

fn part1(input: &str, min: i128, max: i128) -> usize {
    let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();

    let combinations = hailstones
        .into_iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    let intersections = combinations
        .into_iter()
        .filter_map(|(a, b)| {
            if let Some((x, y)) = a.intersection(&b) {
                // intersection is in past
                if i128::signum(x - a.x) != i128::signum(a.dx)
                    || i128::signum(x - b.x) != i128::signum(b.dx)
                    || i128::signum(y - a.y) != i128::signum(a.dy)
                    || i128::signum(y - b.y) != i128::signum(b.dy)
                {
                    None
                } else if x >= min && x <= max && y >= min && y <= max {
                    // intersection is in the specified area
                    Some((x, y))
                } else {
                    // intersection is not in the specified area
                    None
                }
            } else {
                // no intersection
                None
            }
        })
        .collect::<Vec<_>>();

    intersections.len()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Hailstone {
    x: i128,
    y: i128,
    z: i128,
    dx: i128,
    dy: i128,
    dz: i128,
}
impl Hailstone {
    fn new(x: i128, y: i128, z: i128, dx: i128, dy: i128, dz: i128) -> Self {
        Self {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }

    fn to_line(self) -> (i128, i128, i128) {
        let a = self.dy;
        let b = -self.dx;
        let c = self.dx * self.y - self.dy * self.x;
        (a, b, c)
    }

    fn intersection(&self, other: &Self) -> Option<(i128, i128)> {
        let (a1, b1, c1) = self.to_line();
        let (a2, b2, c2) = other.to_line();

        if (a1 * b2 - a2 * b1) == 0 {
            return None;
        }

        let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        Some((x, y))
    }
}
impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (pos, vel) = value.split_once(" @ ").expect("no @ symbol");
        let mut pos = pos.split(", ");
        let mut vel = vel.split(", ");

        let x = pos
            .next()
            .expect("no x")
            .trim()
            .parse::<i128>()
            .expect("x not a number");
        let y = pos
            .next()
            .expect("no y")
            .trim()
            .parse::<i128>()
            .expect("y not a number");
        let z = pos
            .next()
            .expect("no z")
            .trim()
            .parse::<i128>()
            .expect("z not a number");
        let dx = vel
            .next()
            .expect("no dx")
            .trim()
            .parse::<i128>()
            .expect("dx not a number");
        let dy = vel
            .next()
            .expect("no dy")
            .trim()
            .parse::<i128>()
            .expect("dy not a number");
        let dz = vel
            .next()
            .expect("no dz")
            .trim()
            .parse::<i128>()
            .expect("dz not a number");

        Hailstone::new(x, y, z, dx, dy, dz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                indoc!(
                    "
                    19, 13, 30 @ -2,  1, -2
                    18, 19, 22 @ -1, -1, -2
                    20, 25, 34 @ -2, -2, -4
                    12, 31, 28 @ -1, -2, -1
                    20, 19, 15 @  1, -5, -3
                    "
                ),
                7,
                27
            ),
            2
        );
    }
}
