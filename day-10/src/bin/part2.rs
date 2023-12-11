use std::{collections::BTreeMap, iter};

use itertools::Itertools;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
    let pipes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| Pipe::new(x, y, c.try_into().expect("invalid pipe character")))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let starting_pipe = pipes
        .iter()
        .flatten()
        .find(|p| p.pipe_type == PipeType::Starting)
        .expect("no starting pipe found");

    let mut starting_directions = Vec::new();

    let mut starting = Vec::with_capacity(2);
    if starting_pipe.pipe_type.has_pos_x() {
        if let Some(pipe) = starting_pipe.get_pos_x(&pipes) {
            starting.push((Direction::Left, pipe));
            starting_directions.push(Direction::Right);
        }
    }
    if starting_pipe.pipe_type.has_neg_x() {
        if let Some(pipe) = starting_pipe.get_neg_x(&pipes) {
            starting.push((Direction::Right, pipe));
            starting_directions.push(Direction::Left);
        }
    }
    if starting_pipe.pipe_type.has_pos_y() {
        if let Some(pipe) = starting_pipe.get_pos_y(&pipes) {
            starting.push((Direction::Up, pipe));
            starting_directions.push(Direction::Down);
        }
    }
    if starting_pipe.pipe_type.has_neg_y() {
        if let Some(pipe) = starting_pipe.get_neg_y(&pipes) {
            starting.push((Direction::Down, pipe));
            starting_directions.push(Direction::Up);
        }
    }

    let (path_a, path_b) = starting
        .into_iter()
        .map(|p| iter::successors(Some(p), |(d, p)| p.successor(*d, &pipes)))
        .collect_tuple()
        .expect("more than 2 paths");

    let starting_pipe_type = match starting_directions
        .into_iter()
        .collect_tuple()
        .expect("more than than two paths")
    {
        (Direction::Down, Direction::Up) => PipeType::Vertical,
        (Direction::Right, Direction::Left) => PipeType::Horizontal,
        (Direction::Right, Direction::Down) => PipeType::SouthEast,
        (Direction::Right, Direction::Up) => PipeType::NorthEast,
        (Direction::Left, Direction::Down) => PipeType::SouthWest,
        (Direction::Left, Direction::Up) => PipeType::NorthWest,
        _ => unreachable!("pipe type does not exist"),
    };

    let mut path_elements = BTreeMap::new();
    path_elements.insert((starting_pipe.x, starting_pipe.y), starting_pipe_type);

    let (a, b): (Vec<_>, Vec<_>) = path_a
        .zip(path_b)
        .take_while(|((_, pipe_a), (_, pipe_b))| pipe_a != pipe_b)
        .unzip();
    let mut path = a.into_iter().chain(b).collect::<Vec<_>>();
    let (last_direction, last_pipe) = path.last().expect("no last element");
    path.push(
        last_pipe
            .successor(*last_direction, &pipes)
            .expect("no successor"),
    );
    path.iter().for_each(|(_, pipe)| {
        path_elements.insert((pipe.x, pipe.y), pipe.pipe_type);
    });

    let width = pipes.get(0).expect("no pipes").len();

    let inner = pipes
        .iter()
        .flatten()
        .filter(|p| {
            if path_elements.contains_key(&(p.x, p.y)) {
                false
            } else {
                let amount = (p.x..width)
                    .filter(|x| {
                        let pipe = path_elements.get(&(*x, p.y));
                        pipe.map(|pipe| {
                            matches!(
                                *pipe,
                                PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest
                            )
                        })
                        .unwrap_or(false)
                    })
                    .count();

                amount % 2 == 1
            }
        })
        .collect::<Vec<_>>();

    inner.len()
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Pipe {
    x: usize,
    y: usize,
    pipe_type: PipeType,
}
impl Pipe {
    fn new(x: usize, y: usize, pipe_type: PipeType) -> Self {
        Self { x, y, pipe_type }
    }

    fn successor<'a>(
        &self,
        entry_direction: Direction,
        pipes: &'a [Vec<Pipe>],
    ) -> Option<(Direction, &'a Self)> {
        if entry_direction != Direction::Right && self.pipe_type.has_pos_x() {
            self.get_pos_x(pipes).map(|p| (Direction::Left, p))
        } else if entry_direction != Direction::Left && self.pipe_type.has_neg_x() {
            self.get_neg_x(pipes).map(|p| (Direction::Right, p))
        } else if entry_direction != Direction::Up && self.pipe_type.has_neg_y() {
            self.get_neg_y(pipes).map(|p| (Direction::Down, p))
        } else if entry_direction != Direction::Down && self.pipe_type.has_pos_y() {
            self.get_pos_y(pipes).map(|p| (Direction::Up, p))
        } else {
            None
        }
    }

    fn get_pos_x<'a>(&self, pipes: &'a [Vec<Pipe>]) -> Option<&'a Pipe> {
        if self.pipe_type.has_pos_x() {
            pipes
                .get(self.y)?
                .get(self.x + 1)
                .filter(|p| p.pipe_type.has_neg_x())
        } else {
            None
        }
    }
    fn get_neg_x<'a>(&self, pipes: &'a [Vec<Pipe>]) -> Option<&'a Pipe> {
        if self.pipe_type.has_neg_x() {
            pipes
                .get(self.y)?
                .get(self.x.checked_sub(1)?)
                .filter(|p| p.pipe_type.has_pos_x())
        } else {
            None
        }
    }
    fn get_pos_y<'a>(&self, pipes: &'a [Vec<Pipe>]) -> Option<&'a Pipe> {
        if self.pipe_type.has_pos_y() {
            pipes
                .get(self.y + 1)?
                .get(self.x)
                .filter(|p| p.pipe_type.has_neg_y())
        } else {
            None
        }
    }
    fn get_neg_y<'a>(&self, pipes: &'a [Vec<Pipe>]) -> Option<&'a Pipe> {
        if self.pipe_type.has_neg_y() {
            pipes
                .get(self.y.checked_sub(1)?)?
                .get(self.x)
                .filter(|p| p.pipe_type.has_pos_y())
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum PipeType {
    Empty,
    Starting,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl PipeType {
    fn has_pos_x(&self) -> bool {
        matches!(
            self,
            Self::Horizontal | Self::NorthEast | Self::SouthEast | Self::Starting
        )
    }
    fn has_neg_x(&self) -> bool {
        matches!(
            self,
            Self::Horizontal | Self::NorthWest | Self::SouthWest | Self::Starting
        )
    }
    fn has_pos_y(&self) -> bool {
        matches!(
            self,
            Self::Vertical | Self::SouthEast | Self::SouthWest | Self::Starting
        )
    }
    fn has_neg_y(&self) -> bool {
        matches!(
            self,
            Self::Vertical | Self::NorthEast | Self::NorthWest | Self::Starting
        )
    }
}

impl TryFrom<char> for PipeType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Starting),
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part2() {
        // assert_eq!(
        //     part2(indoc!(
        //         "
        //         FF7FSF7F7F7F7F7F---7
        //         L|LJ||||||||||||F--J
        //         FL-7LJLJ||||||LJL-77
        //         F--JF--7||LJLJ7F7FJ-
        //         L---JF-JLJ.||-FJLJJ7
        //         |F|F-JF---7F7-L7L|7|
        //         |FFJF7L7F-JF7|JL---7
        //         7-L-JL7||F7|L7F-7F7|
        //         L.L7LFJ|||||FJL7||LJ
        //         L7JLJL-JLJLJL--JLJ.L
        //         "
        //     )),
        //     10
        // );

        assert_eq!(
            part2(indoc!(
                "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
            "
            )),
            8
        )
    }
}
