use nom::{branch::alt, bytes::complete::is_not, character::complete::digit1, multi::many1};
use std::collections::HashSet;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u32 {
    let mut symbols = HashSet::new();
    input
        .lines()
        .enumerate()
        .flat_map(|l| l.1.chars().enumerate().map(move |c| (l.0, c.0, c.1)))
        .filter(|(_, _, c)| !c.is_ascii_digit() && *c != '.')
        .for_each(|(y, x, _)| {
            symbols.insert((x, y));
        });

    input
        .lines()
        .enumerate()
        .map(|(y, l)| line(l, y, &symbols))
        .sum()
}

fn line(i: &str, y: usize, symbols: &HashSet<(usize, usize)>) -> u32 {
    many1(alt((
        is_not("0123456789"),
        digit1::<&str, nom::error::Error<&str>>,
    )))(i)
    .unwrap()
    .1
    .into_iter()
    .fold((0, 0), |(pos, sum), cur| {
        if let Ok(num) = cur.parse::<u32>() {
            let y_minus_1 = y.checked_sub(1).unwrap_or(y + 1);
            let pos_minus_1 = usize::checked_sub(pos, 1).unwrap_or(pos);

            let symbol_y = (pos_minus_1..=pos + cur.len())
                .any(|x| symbols.contains(&(x, y + 1)) || symbols.contains(&(x, y_minus_1)));
            let symbol_x =
                symbols.contains(&(pos_minus_1, y)) || symbols.contains(&(pos + cur.len(), y));

            let value = if symbol_y || symbol_x { num } else { 0 };

            (pos + cur.len(), sum + value)
        } else {
            (pos + cur.len(), sum)
        }
    })
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."), 4361);
    }
}
