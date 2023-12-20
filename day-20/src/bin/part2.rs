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
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> usize {
    let raw_modules = input_parser(input).unwrap().1;
    let conjunction_targets = raw_modules
        .iter()
        .filter(|(_, mod_type, _)| mod_type == &Module::Conjunction)
        .map(|(tag, _, _)| {
            (
                *tag,
                raw_modules
                    .iter()
                    .filter(|(_, _, targets)| targets.contains(tag))
                    .map(|(tag, _, _)| *tag)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let modules = to_stateful_modules(&raw_modules, &conjunction_targets);

    let starts = follow_signal(("button", false, "broadcaster"), &modules);

    println!("{:?}", starts);

    starts
        .iter()
        .map(|(_, i)| *i)
        .reduce(num::integer::lcm)
        .expect("empty list")
}

fn to_stateful_modules<'a>(
    modules: &'a [(&'a str, Module, Vec<&'a str>)],
    conjunction_targets: &HashMap<&str, Vec<&'a str>>,
) -> HashMap<&'a str, StatefulModule<'a, 'a, 'a>> {
    modules
        .iter()
        .map(|(tag, mod_type, targets)| {
            let module = match mod_type {
                Module::Broadcaster => StatefulModule::Broadcaster {
                    targets: targets.clone(),
                },
                Module::FlipFlop => StatefulModule::FlipFlop {
                    targets: targets.clone(),
                    state: Mutex::new(false),
                },
                Module::Conjunction => StatefulModule::Conjunction {
                    targets: targets.clone(),
                    state: conjunction_targets[tag]
                        .iter()
                        .map(|tag| (*tag, Mutex::new(false)))
                        .collect(),
                },
            };
            (*tag, module)
        })
        .collect()
}

fn follow_signal<'a>(
    start: (&'a str, bool, &'a str),
    modules: &'a HashMap<&'a str, StatefulModule<'a, 'a, 'a>>,
) -> Vec<(&'a str, usize)> {
    let mut res = Vec::new();
    let mut i = 0;
    'button: loop {
        i += 1;
        let mut next_state = VecDeque::from(vec![start]);
        while let Some((prev_tag, high, tag)) = next_state.pop_front() {
            if !high && ["pv", "qh", "xm", "hz"].contains(&tag) {
                res.push((tag, i));
            }
            if res.len() >= 4 {
                break 'button;
            }
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
    res
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjunction,
}
