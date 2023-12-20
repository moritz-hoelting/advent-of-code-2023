use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    sync::Mutex,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> usize {
    let modules = input_parser(input).unwrap().1;
    let conjunction_targets = modules
        .iter()
        .filter(|(_, mod_type, _)| mod_type == &Module::Conjunction)
        .map(|(tag, _, _)| {
            (
                *tag,
                modules
                    .iter()
                    .filter(|(_, _, targets)| targets.contains(tag))
                    .map(|(tag, _, _)| *tag)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let modules = modules
        .into_iter()
        .map(|(tag, mod_type, targets)| {
            let module = match mod_type {
                Module::Broadcaster => StatefulModule::Broadcaster { targets },
                Module::FlipFlop => StatefulModule::FlipFlop {
                    targets,
                    state: Mutex::new(false),
                },
                Module::Conjunction => StatefulModule::Conjunction {
                    targets,
                    state: conjunction_targets[&tag]
                        .iter()
                        .map(|tag| (*tag, Mutex::new(false)))
                        .collect(),
                },
            };
            (tag, module)
        })
        .collect::<HashMap<_, _>>();

    let counter = PulseCounter::new();

    for _ in 0..1000 {
        let mut next_state = VecDeque::from(vec![("button", false, "broadcaster")]);
        while let Some((prev_tag, high, tag)) = next_state.pop_front() {
            counter.count(high);
            if let Some(module) = modules.get(tag) {
                next_state.extend(
                    module
                        .send(high, prev_tag)
                        .into_iter()
                        .map(|(t, h)| (tag, h, t)),
                );
            }
        }
    }

    counter.get_product()
}

type ModuleVec<'a> = Vec<(&'a str, Module, Vec<&'a str>)>;
fn input_parser(i: &str) -> IResult<&str, ModuleVec> {
    map(separated_list1(line_ending, module_parser), |modules| {
        modules
            .into_iter()
            .map(|((mod_type, tag), targets)| (tag, mod_type, targets))
            .collect::<Vec<_>>()
    })(i)
}

fn module_parser(i: &str) -> IResult<&str, ((Module, &str), Vec<&str>)> {
    separated_pair(
        alt((
            value((Module::Broadcaster, "broadcaster"), tag("broadcaster")),
            pair(value(Module::FlipFlop, complete::char('%')), alpha1),
            pair(value(Module::Conjunction, complete::char('&')), alpha1),
        )),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(i)
}

#[derive(Debug)]
enum StatefulModule<'a, 'b, 'c> {
    Broadcaster {
        targets: Vec<&'a str>,
    },
    FlipFlop {
        targets: Vec<&'b str>,
        state: Mutex<bool>,
    },
    Conjunction {
        targets: Vec<&'c str>,
        state: BTreeMap<&'c str, Mutex<bool>>,
    },
}
impl StatefulModule<'_, '_, '_> {
    fn send(&self, high: bool, prev_tag: &str) -> Vec<(&str, bool)> {
        match self {
            Self::Broadcaster { targets } => {
                targets.iter().map(|tag| (*tag, high)).collect::<Vec<_>>()
            }
            Self::FlipFlop { targets, state } => {
                if high {
                    Vec::new()
                } else {
                    let mut state = state.lock().unwrap();
                    *state = !*state;
                    let high = *state;
                    drop(state);
                    targets.iter().map(|tag| (*tag, high)).collect::<Vec<_>>()
                }
            }
            Self::Conjunction { targets, state } => {
                let mut sender_state = state[prev_tag].lock().unwrap();
                *sender_state = high;
                drop(sender_state);
                let all_high = state.values().all(|state| *state.lock().unwrap());
                targets
                    .iter()
                    .map(|tag| (*tag, !all_high))
                    .collect::<Vec<_>>()
            }
        }
    }
}

#[derive(Debug)]
struct PulseCounter {
    count_low: Mutex<usize>,
    count_high: Mutex<usize>,
}
impl PulseCounter {
    fn new() -> Self {
        Self {
            count_low: Mutex::new(0),
            count_high: Mutex::new(0),
        }
    }
    fn count(&self, high: bool) {
        if high {
            *self.count_high.lock().unwrap() += 1;
        } else {
            *self.count_low.lock().unwrap() += 1;
        }
    }

    fn get(&self, high: bool) -> usize {
        if high {
            *self.count_high.lock().unwrap()
        } else {
            *self.count_low.lock().unwrap()
        }
    }

    fn get_product(&self) -> usize {
        self.get(true) * self.get(false)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            part1(indoc!(
                "
                broadcaster -> a, b, c
                %a -> b
                %b -> c
                %c -> inv
                &inv -> a
                "
            )),
            32_000_000
        );
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            part1(indoc!(
                "
                broadcaster -> a
                %a -> inv, con
                &inv -> b
                %b -> con
                &con -> output
                "
            )),
            11_687_500
        );
    }
}
