fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<_>>>();

    let start_x = map
        .first()
        .and_then(|row| row.iter().position(|tile| *tile == Tile::Path))
        .expect("No start found");
    let end_x = map
        .last()
        .and_then(|row| row.iter().position(|tile| *tile == Tile::Path))
        .expect("No end found");

    let width = map[0].len();
    let pos_to_index = move |(x, y)| x + y * width;
    let index_to_pos = move |index| (index % width, index / width);

    let mut node_map = vec![None; map.len() * map[0].len()];

    *node_map.get_mut(start_x).unwrap() = Some(0);
    *node_map
        .get_mut(pos_to_index((end_x, map.len() - 1)))
        .unwrap() = Some(1);

    let start = (start_x, 0_usize);

    let mut graph = vec![Node { edges: Vec::new() }; 2];
    let mut directions_exited = vec![0; 2];
    let mut open_set = vec![(0, start, Direction::Down)];
    let mut next_steps = Vec::new();
    let mut nearest_goal = 0;

    while let Some((start_node, start_pos, start_direction)) = open_set.pop() {
        // if already exited this node in this direction, skip it
        if directions_exited[start_node] & start_direction.bit() != 0 {
            continue;
        }
        let mut pos = start_direction.increment_position(start_pos);
        let mut direction = start_direction;
        let mut index = pos_to_index(pos);
        let mut length = 1;

        // walk until hitting a dead end or a node
        loop {
            if let Some(end_node) = node_map[index] {
                if end_node == 1 {
                    nearest_goal = start_node;
                }
                // save the new direction exited for the nodes
                directions_exited[start_node] |= start_direction.bit();
                directions_exited[end_node] |= direction.invert().bit();
                // add the edge to the graph
                graph[start_node].edges.push((end_node, length));
                graph[end_node].edges.push((start_node, length));
                break;
            }

            // check every direction
            for new_direction in Direction::ALL {
                // don't go back the same way
                if new_direction.invert() == direction {
                    continue;
                }
                // go one step in the new direction
                let new_pos = new_direction.increment_position(pos);
                let new_index = pos_to_index(new_pos);
                let (new_index_x, new_index_y) = index_to_pos(new_index);
                // if not a path, skip it
                if map[new_index_y][new_index_x] == Tile::Forest {
                    continue;
                }
                next_steps.push((new_index, new_pos, new_direction));
            }

            // if only one direction is available, continue walking
            if next_steps.len() == 1 {
                let (new_index, new_pos, new_direction) = next_steps.pop().unwrap();
                index = new_index;
                pos = new_pos;
                direction = new_direction;
                length += 1;
            } else {
                // otherwise, create a new node
                let end_node = graph.len();
                node_map[index] = Some(end_node);
                graph.push(Node { edges: Vec::new() });
                directions_exited.push(0);
                directions_exited[start_node] |= start_direction.bit();
                directions_exited[end_node] |= direction.invert().bit();
                graph[start_node].edges.push((end_node, length));
                graph[end_node].edges.push((start_node, length));

                open_set.extend(
                    next_steps
                        .iter()
                        .map(|(_, _, direction)| (end_node, pos, *direction)),
                );
                next_steps.clear();
                break;
            }
        }
    }

    longest_path(&graph, 0, nearest_goal, 1, &mut Vec::new()).unwrap()
}

fn longest_path(
    graph: &Vec<Node>,
    current_node: usize,
    nearest_goal: usize,
    goal: usize,
    visited: &mut Vec<usize>,
) -> Option<usize> {
    graph[current_node]
        .edges
        .iter()
        .filter_map(|&(next_node, length)| {
            if next_node == goal {
                Some(length)
            } else if current_node == nearest_goal && next_node != goal
                || visited.contains(&next_node)
            {
                None
            } else {
                visited.push(next_node);
                let result = longest_path(graph, next_node, nearest_goal, goal, visited);
                visited.pop();
                result.map(|x| x + length)
            }
        })
        .max()
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Right, Self::Down, Self::Left];

    fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
    fn increment_position(&self, (x, y): Coord) -> Coord {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    fn bit(&self) -> u8 {
        1 << (*self as u8)
    }
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' | '>' | '<' | '^' | 'v' => Self::Path,
            _ => panic!("Invalid tile: {}", value),
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
                #.#####################
                #.......#########...###
                #######.#########.#.###
                ###.....#.>.>.###.#.###
                ###v#####.#v#.###.#.###
                ###.>...#.#.#.....#...#
                ###v###.#.#.#########.#
                ###...#.#.#.......#...#
                #####.#.#.#######.#.###
                #.....#.#.#.......#...#
                #.#####.#.#.#########v#
                #.#...#...#...###...>.#
                #.#.#v#######v###.###v#
                #...#.>.#...>.>.#.###.#
                #####v#.#.###v#.#.###.#
                #.....#...#...#.#.#...#
                #.#########.###.#.#.###
                #...###...#...#...#.###
                ###.###.#.###v#####v###
                #...#...#.#.>.>.#.>.###
                #.###.###.#.###.#.#v###
                #.....###...###...#...#
                #####################.#
                "
            )),
            154
        );
    }
}
