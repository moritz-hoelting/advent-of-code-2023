use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

fn main() {
    println!("{}", part2(include_str!("./input.txt"),));
}

fn part2(input: &str) -> i64 {
    let hailstones = input.lines().map(Hailstone::from).collect::<Vec<_>>();

    let cfg = Config::new();
    let context = Context::new(&cfg);
    let solver = Solver::new(&context);

    let x = Int::new_const(&context, "x");
    let y = Int::new_const(&context, "y");
    let z = Int::new_const(&context, "z");
    let dx = Int::new_const(&context, "dx");
    let dy = Int::new_const(&context, "dy");
    let dz = Int::new_const(&context, "dz");

    // add assertions for every hailstone
    for (i, hailstone) in hailstones.iter().take(3).enumerate() {
        let a = Int::from_i64(&context, hailstone.x.try_into().unwrap());
        let da = Int::from_i64(&context, hailstone.dx.try_into().unwrap());
        let b = Int::from_i64(&context, hailstone.y.try_into().unwrap());
        let db = Int::from_i64(&context, hailstone.dy.try_into().unwrap());
        let c = Int::from_i64(&context, hailstone.z.try_into().unwrap());
        let dc = Int::from_i64(&context, hailstone.dz.try_into().unwrap());

        // add assertions that the hailstone is at the given position at time t
        let t = Int::new_const(&context, "t".to_string() + &i.to_string());
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + dx.clone() * t.clone())._eq(&(a + da * t.clone())));
        solver.assert(&(y.clone() + dy.clone() * t.clone())._eq(&(b + db * t.clone())));
        solver.assert(&(z.clone() + dz.clone() * t.clone())._eq(&(c + dc * t.clone())));
    }

    if solver.check() == SatResult::Sat {
        if let Some(m) = solver.get_model() {
            // get the sum of the x,y,z coordinates of the starting position
            m.eval(&(x + y + z), true).unwrap().as_i64().unwrap()
        } else {
            panic!("Failed to solve!");
        }
    } else {
        panic!("Failed to check")
    }
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
    fn test_part2() {
        assert_eq!(
            part2(indoc!(
                "
                    19, 13, 30 @ -2,  1, -2
                    18, 19, 22 @ -1, -1, -2
                    20, 25, 34 @ -2, -2, -4
                    12, 31, 28 @ -1, -2, -1
                    20, 19, 15 @  1, -5, -3
                    "
            )),
            47
        );
    }
}
