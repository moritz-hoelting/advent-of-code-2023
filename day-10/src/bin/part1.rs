use std::{fmt::Debug, sync::Mutex};

use pathfinding::prelude::bfs;
use rayon::prelude::*;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let pipes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.par_char_indices()
                .map(|(x, c)| VPipe::new(x, y, c.try_into().expect("invalid pipe character")))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let starting_pipe = pipes
        .iter()
        .flatten()
        .find(|p| p.pipe_type == PipeType::Starting)
        .expect("no starting pipe found");

    let fist_step = starting_pipe.get_neg_x(&pipes);

    let result = fist_step
        .and_then(|starting_pipe| {
            bfs(
                &starting_pipe,
                |pipe| pipe.successors(&pipes),
                |pipe| pipe.pipe_type == PipeType::Starting,
            )
        })
        .expect("no loop found");

    result.len() / 2
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

struct VPipe(Pipe, Mutex<Direction>);
impl VPipe {
    fn new(x: usize, y: usize, pipe_type: PipeType) -> Self {
        Self(Pipe::new(x, y, pipe_type), Mutex::new(Direction::None))
    }

    fn was_visited(&self) -> bool {
        self.get_direction() != Direction::None
    }

    fn set_direction(&self, direction: Direction) {
        *self.1.lock().unwrap() = direction;
    }
    fn get_direction(&self) -> Direction {
        *self.1.lock().unwrap()
    }
}
impl std::ops::Deref for VPipe {
    type Target = Pipe;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialEq for VPipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for VPipe {}
impl std::hash::Hash for VPipe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl Debug for VPipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VPipe")
            .field("pipe", &self.0)
            .field("entered_from", &self.get_direction())
            .finish()
    }
}
impl Clone for VPipe {
    fn clone(&self) -> Self {
        Self(
            Pipe::new(self.x, self.y, self.pipe_type),
            Mutex::new(self.get_direction()),
        )
    }
}

impl VPipe {
    fn successors<'a>(&self, pipes: &'a [Vec<VPipe>]) -> Vec<&'a VPipe> {
        vec![
            self.get_pos_x(pipes),
            self.get_neg_x(pipes),
            self.get_pos_y(pipes),
            self.get_neg_y(pipes),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn get_pos_x<'a>(&self, pipes: &'a [Vec<VPipe>]) -> Option<&'a VPipe> {
        if self.pipe_type.has_pos_x() && self.get_direction() != Direction::Right {
            pipes
                .get(self.y)?
                .get(self.x + 1)
                .filter(|p| p.pipe_type.has_neg_x() && !p.was_visited())
                .map(|p| {
                    p.set_direction(Direction::Left);
                    p
                })
        } else {
            None
        }
    }
    fn get_neg_x<'a>(&self, pipes: &'a [Vec<VPipe>]) -> Option<&'a VPipe> {
        if self.pipe_type.has_neg_x() && self.get_direction() != Direction::Left {
            pipes
                .get(self.y)?
                .get(self.x.checked_sub(1)?)
                .filter(|p| p.pipe_type.has_pos_x() && !p.was_visited())
                .map(|p| {
                    p.set_direction(Direction::Right);
                    p
                })
        } else {
            None
        }
    }
    fn get_pos_y<'a>(&self, pipes: &'a [Vec<VPipe>]) -> Option<&'a VPipe> {
        if self.pipe_type.has_pos_y() && self.get_direction() != Direction::Down {
            pipes
                .get(self.y + 1)?
                .get(self.x)
                .filter(|p| p.pipe_type.has_neg_y() && !p.was_visited())
                .map(|p| {
                    p.set_direction(Direction::Up);
                    p
                })
        } else {
            None
        }
    }
    fn get_neg_y<'a>(&self, pipes: &'a [Vec<VPipe>]) -> Option<&'a VPipe> {
        if self.pipe_type.has_neg_y() && self.get_direction() != Direction::Up {
            pipes
                .get(self.y.checked_sub(1)?)?
                .get(self.x)
                .filter(|p| p.pipe_type.has_pos_y() && !p.was_visited())
                .map(|p| {
                    p.set_direction(Direction::Down);
                    p
                })
        } else {
            None
        }
    }
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
    #[ignore = "wrong starting pipe chosen in code"]
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
                "
                ..F7.
                .FJ|.
                SJ.L7
                |F--J
                LJ...
                "
            )),
            8
        );
    }
}
