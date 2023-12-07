use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u64 {
    let hands = input.lines().map(process_hand).collect::<Vec<_>>();

    calc_total_winnings(hands)
}

fn process_hand(input: &str) -> Hand {
    let (hand, bid) = input.split_once(' ').expect("invalid input");

    let cards = hand.chars().map(Card::from).collect::<Vec<_>>();
    let mut card_amounts = cards.clone();
    card_amounts.sort();
    card_amounts.reverse();
    let grouped_cards = card_amounts.into_iter().group_by(|c| *c);
    let mut grouped_cards = grouped_cards
        .into_iter()
        .filter(|(c, _)| *c != Card::J)
        .map(|(c, g)| (c, g.count()))
        .collect::<Vec<_>>();
    grouped_cards.sort_by(|(c1, g1), (c2, g2)| match g2.cmp(g1) {
        Ordering::Equal => c2.cmp(c1),
        x => x,
    });

    let jokers = cards.iter().filter(|c| **c == Card::J).count();

    let mut grouped_cards = grouped_cards.into_iter();

    let rank = if let Some((_, a)) = grouped_cards.next() {
        match a + jokers {
            5 => Rank::FiveOfAKind,
            4 => Rank::FourOfAKind,
            3 => match grouped_cards.next() {
                Some((_, 2)) => Rank::FullHouse,
                Some((_, 1)) => Rank::ThreeOfAKind,
                _ => panic!("invalid hand"),
            },
            2 => match grouped_cards.next() {
                Some((_, 2)) => Rank::TwoPair,
                Some((_, 1)) => Rank::OnePair,
                _ => panic!("invalid hand"),
            },
            1 => Rank::HighCard,
            _ => panic!("invalid hand"),
        }
    } else if jokers == 5 {
        Rank::FiveOfAKind
    } else {
        panic!("invalid hand")
    };

    Hand {
        bid: bid.parse().expect("invalid bid"),
        cards: cards.try_into().expect("invalid hand"),
        rank,
    }
}

fn calc_total_winnings(mut hands: Vec<Hand>) -> u64 {
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid as u64)
        .sum()
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
struct Hand {
    rank: Rank,
    cards: [Card; 5],
    bid: u32,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        process_hand(s)
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
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
            )),
            5905
        );
    }
}
