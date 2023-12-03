use nom::{branch::alt, bytes::complete::is_not, character::complete::digit1, multi::many1};
use std::{collections::HashMap, sync::Mutex};

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u32 {
    let mut gears = HashMap::new();
    input
        .lines()
        .enumerate()
        .flat_map(|l| l.1.chars().enumerate().map(move |c| (l.0, c.0, c.1)))
        .filter(|(_, _, c)| *c == '*')
        .for_each(|(y, x, _)| {
            gears.insert((x, y), Mutex::new(Vec::new()));
        });

    input
        .lines()
        .enumerate()
        .for_each(|(y, l)| line(l, y, &gears));

    gears
        .values()
        .map(|nums_mutex| {
            let mut nums = nums_mutex.lock().unwrap();
            nums.sort();
            nums.dedup();
            if nums.len() == 2 {
                nums[0] * nums[1]
            } else {
                0
            }
        })
        .sum()
}

fn line(i: &str, y: usize, gears: &HashMap<(usize, usize), Mutex<Vec<u32>>>) {
    many1(alt((
        is_not("0123456789"),
        digit1::<&str, nom::error::Error<&str>>,
    )))(i)
    .unwrap()
    .1
    .into_iter()
    .fold(0, |pos, cur| {
        if let Ok(num) = cur.parse::<u32>() {
            let y_minus_1 = y.checked_sub(1).unwrap_or(y + 1);
            let pos_minus_1 = usize::checked_sub(pos, 1).unwrap_or(pos);

            (pos_minus_1..=pos + cur.len()).for_each(|x| {
                if let Some(v) = gears.get(&(x, y + 1)) {
                    v.lock().unwrap().push(num)
                }
                if let Some(v) = gears.get(&(x, y_minus_1)) {
                    v.lock().unwrap().push(num)
                }
            });

            if let Some(v) = gears.get(&(pos_minus_1, y)) {
                v.lock().unwrap().push(num)
            }
            if let Some(v) = gears.get(&(pos + cur.len(), y)) {
                v.lock().unwrap().push(num)
            }
        }
        pos + cur.len()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 467835);
    }
}
