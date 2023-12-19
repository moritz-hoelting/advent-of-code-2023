use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::{map, value},
    multi::{count, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u32 {
    let (workflows, parts) = input_parser(input).expect("invalid input").1;

    let starting_workflow = workflows.get("in").expect("no starting workflow");

    parts
        .into_iter()
        .filter(|p| {
            let mut current_workflow = starting_workflow;
            let mut current_rule = 0;
            loop {
                let rule = current_workflow.rules.get(current_rule);
                if let Some(rule) = rule {
                    let part_value = p.get(rule.category);
                    let condition = rule.condition;
                    if (condition == Condition::GreaterThan && part_value > rule.value)
                        || (condition == Condition::LessThan && part_value < rule.value)
                    {
                        if rule.target == "A" {
                            return true;
                        }
                        if rule.target == "R" {
                            return false;
                        }
                        current_workflow =
                            workflows.get(rule.target).expect("invalid target workflow");
                        current_rule = 0;
                    } else {
                        current_rule += 1;
                    }
                } else {
                    if current_workflow.finally == "A" {
                        return true;
                    }
                    if current_workflow.finally == "R" {
                        return false;
                    }
                    current_workflow = workflows
                        .get(current_workflow.finally)
                        .expect("invalid finally workflow");
                    current_rule = 0;
                }
            }
        })
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}

type WorflowsAndParts<'a> = (HashMap<&'a str, Workflow<'a, 'a>>, Vec<Part>);
fn input_parser(i: &str) -> IResult<&str, WorflowsAndParts> {
    separated_pair(
        map(
            separated_list1(line_ending, workflow_parser),
            HashMap::from_iter,
        ),
        count(line_ending, 2),
        separated_list1(line_ending, part_parser),
    )(i)
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
            complete::u32,
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

fn part_parser(i: &str) -> IResult<&str, Part> {
    map(
        delimited(
            complete::char::<&str, nom::error::Error<&str>>('{'),
            separated_list1(
                complete::char(','),
                separated_pair(category_parser, complete::char('='), complete::u32),
            ),
            complete::char('}'),
        ),
        |v| {
            Part::new(
                v.iter()
                    .find(|(c, _)| c == &Category::ExtremelyCoolLooking)
                    .unwrap()
                    .1,
                v.iter().find(|(c, _)| c == &Category::Musical).unwrap().1,
                v.iter()
                    .find(|(c, _)| c == &Category::Aerodynamic)
                    .unwrap()
                    .1,
                v.iter().find(|(c, _)| c == &Category::Shiny).unwrap().1,
            )
        },
    )(i)
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
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    pub fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { x, m, a, s }
    }

    pub fn get(&self, category: Category) -> u32 {
        match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
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
    value: u32,
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
    fn test_part1() {
        assert_eq!(
            part1(indoc!(
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
            19114
        );
    }
}
