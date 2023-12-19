use std::collections::{hash_map::RandomState, HashMap};

use nom::{
    branch::alt,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u64 {
    let workflows: HashMap<&str, Workflow<'_, '_>, RandomState> = map(
        separated_list1(line_ending, workflow_parser),
        HashMap::from_iter,
    )(input)
    .expect("invalid input")
    .1;

    let starting_workflow = workflows.get("in").expect("no starting workflow");

    let mut stack = vec![(starting_workflow, Part::new())];

    let mut possibilities = 0;

    while let Some((workflow, part)) = stack.pop() {
        let mut pass_on_part = part;
        for rule in &workflow.rules {
            let mut local_part = pass_on_part;
            let (split_a, split_b) = pass_on_part
                .get(rule.category)
                .split(rule.value, rule.condition);
            pass_on_part.set(rule.category, split_a);
            local_part.set(rule.category, split_b);

            if !local_part.get(rule.category).is_empty() {
                match rule.target {
                    "A" => {
                        possibilities += local_part.combination_count();
                        continue;
                    }
                    "R" => {}
                    target => {
                        stack.push((workflows.get(target).expect("invalid target"), local_part))
                    }
                }
            }
            if local_part.get(rule.category).is_empty() {
                continue;
            }
        }

        match workflow.finally {
            "R" => continue,
            "A" => possibilities += pass_on_part.combination_count(),
            target => stack.push((
                workflows.get(target).expect("invalid finally"),
                pass_on_part,
            )),
        }
    }

    possibilities
}

fn workflow_parser(i: &str) -> IResult<&str, (&str, Workflow)> {
    map(
        pair(
            alpha1,
            delimited(
                complete::char('{'),
                separated_pair(
                    separated_list1(complete::char(','), rule_parser),
                    complete::char(','),
                    alpha1,
                ),
                complete::char('}'),
            ),
        ),
        |(label, (rules, finally))| (label, Workflow { rules, finally }),
    )(i)
}

fn rule_parser(i: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            category_parser,
            alt((
                value(Condition::GreaterThan, complete::char('>')),
                value(Condition::LessThan, complete::char('<')),
            )),
            complete::u16,
            complete::char(':'),
            alpha1,
        )),
        |(category, condition, value, _, target)| Rule {
            category,
            condition,
            value,
            target,
        },
    )(i)
}

fn category_parser(i: &str) -> IResult<&str, Category> {
    map(one_of("xmas"), |c| {
        Category::try_from(c).expect("invalid category")
    })(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}
impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::ExtremelyCoolLooking),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValueRange {
    min: u16,
    max: u16,
}
impl ValueRange {
    fn new(min: u16, max: u16) -> Self {
        Self { min, max }
    }

    fn combination_count(&self) -> u64 {
        (self.max + 1 - self.min) as u64
    }

    fn split(&self, threshold: u16, cond: Condition) -> (ValueRange, ValueRange) {
        match cond {
            Condition::GreaterThan => (
                ValueRange::new(self.min, threshold),
                ValueRange::new(threshold + 1, self.max),
            ),
            Condition::LessThan => (
                ValueRange::new(threshold, self.max),
                ValueRange::new(self.min, threshold - 1),
            ),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: ValueRange,
    m: ValueRange,
    a: ValueRange,
    s: ValueRange,
}
impl Part {
    pub fn new() -> Self {
        Self {
            x: ValueRange::new(1, 4000),
            m: ValueRange::new(1, 4000),
            a: ValueRange::new(1, 4000),
            s: ValueRange::new(1, 4000),
        }
    }

    pub fn get(&self, category: Category) -> ValueRange {
        match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }

    pub fn set(&mut self, category: Category, range: ValueRange) {
        match category {
            Category::ExtremelyCoolLooking => self.x = range,
            Category::Musical => self.m = range,
            Category::Aerodynamic => self.a = range,
            Category::Shiny => self.s = range,
        }
    }

    pub fn combination_count(&self) -> u64 {
        self.s.combination_count()
            * self.a.combination_count()
            * self.m.combination_count()
            * self.x.combination_count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow<'a, 'b> {
    rules: Vec<Rule<'b>>,
    finally: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule<'a> {
    category: Category,
    condition: Condition,
    value: u16,
    target: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    GreaterThan,
    LessThan,
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
                px{a<2006:qkq,m>2090:A,rfg}
                pv{a>1716:R,A}
                lnx{m>1548:A,A}
                rfg{s<537:gd,x>2440:R,A}
                qs{s>3448:A,lnx}
                qkq{x<1416:A,crn}
                crn{x>2662:A,R}
                in{s<1351:px,qqz}
                qqz{s>2770:qs,m<1801:hdj,R}
                gd{a>3333:R,R}
                hdj{m>838:A,pv}
                
                {x=787,m=2655,a=1222,s=2876}
                {x=1679,m=44,a=2067,s=496}
                {x=2036,m=264,a=79,s=2244}
                {x=2461,m=1339,a=466,s=291}
                {x=2127,m=1623,a=2188,s=1013}
                "
            )),
            167409079868000
        );
    }
}
