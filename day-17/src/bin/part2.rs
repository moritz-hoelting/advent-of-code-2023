use std::collections::HashMap;

use priority_queue::PriorityQueue;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u32 {
    let heat_loss = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("invalid char") as u8)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    dijkstra((0, 0), &heat_loss)
}

fn dijkstra((x, y): (usize, usize), heat_loss: &Vec<Vec<u8>>) -> u32 {
    let mut distances = HashMap::new();
    distances.insert(Node::new(x, y, Direction::None, 0), 0);

    let mut queue = PriorityQueue::new();
    queue.push(Node::new(x, y, Direction::None, 0), u32::MAX);

    let height = heat_loss.len();
    let width = heat_loss[0].len();

    for y in 1..height {
        for x in 1..width {
            for d in 0..10 {
                queue.push(Node::new(x, y, Direction::Down, d), 0);
                queue.push(Node::new(x, y, Direction::Up, d), 0);
                queue.push(Node::new(x, y, Direction::Right, d), 0);
                queue.push(Node::new(x, y, Direction::Left, d), 0);
            }
        }
    }

    while !queue.is_empty() {
        let (u, p) = queue.pop().unwrap();
        if p == 0 {
            break;
        }
        let distance_to_u = distances.get(&u).copied();
        if let Some(distance_to_u) = distance_to_u {
            for v in u.neighbors(height, width) {
                let new_distance = distance_to_u + heat_loss[v.y][v.x] as u32;

                if new_distance < *distances.get(&v).unwrap_or(&u32::MAX) {
                    distances.insert(v, new_distance);
                    queue.push(v, u32::MAX - new_distance);
                }
            }
        }
    }

    vec![Direction::Down, Direction::Right]
        .into_iter()
        .flat_map(|direction| {
            (4..=10).map(move |distance| Node::new(width - 1, height - 1, direction, distance))
        })
        .flat_map(|node| distances.get(&node))
        .min()
        .copied()
        .expect("no minimum")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    distance: u8,
}

impl Node {
    pub fn new(x: usize, y: usize, direction: Direction, distance: u8) -> Self {
        Self {
            x,
            y,
            direction,
            distance,
        }
    }

    pub fn with_distance(self, distance: u8) -> Node {
        Self { distance, ..self }
    }

    pub fn neighbors(&self, height: usize, width: usize) -> Vec<Node> {
        let above = self
            .y
            .checked_sub(1)
            .map(|y| Node::new(self.x, y, Direction::Up, 1));
        let below =
            (self.y + 1 < height).then_some(Node::new(self.x, self.y + 1, Direction::Down, 1));
        let left = self
            .x
            .checked_sub(1)
            .map(|x| Node::new(x, self.y, Direction::Left, 1));
        let right =
            (self.x + 1 < width).then_some(Node::new(self.x + 1, self.y, Direction::Right, 1));
        let mut neighbors = Vec::new();
        if self.distance >= 4 && self.distance <= 10 {
            match self.direction {
                Direction::Down | Direction::Up => vec![left, right],
                Direction::Left | Direction::Right => vec![above, below],
                Direction::None => vec![above, below, left, right],
            }
            .into_iter()
            .flatten()
            .for_each(|node| neighbors.push(node));
        }

        if self.distance < 10 {
            match self.direction {
                Direction::Down => {
                    if let Some(node) = below {
                        neighbors.push(node.with_distance(self.distance + 1));
                    }
                }
                Direction::Up => {
                    if let Some(node) = above {
                        neighbors.push(node.with_distance(self.distance + 1));
                    }
                }
                Direction::Left => {
                    if let Some(node) = left {
                        neighbors.push(node.with_distance(self.distance + 1));
                    }
                }
                Direction::Right => {
                    if let Some(node) = right {
                        neighbors.push(node.with_distance(self.distance + 1));
                    }
                }
                Direction::None => vec![above, below, left, right]
                    .into_iter()
                    .flatten()
                    .for_each(|node| neighbors.push(node.with_distance(self.distance + 1))),
            }
        }

        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part2_a() {
        assert_eq!(
            part2(indoc!(
                "
                2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
                "
            )),
            94
        );
    }

    #[test]
    fn test_part2_b() {
        assert_eq!(
            part2(indoc!(
                "
                111111111111
                999999999991
                999999999991
                999999999991
                999999999991
                "
            )),
            71
        );
    }
}
