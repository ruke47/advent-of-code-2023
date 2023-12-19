use std::cmp::{max, min};
use std::collections::{HashMap};
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use crate::TestType::{*};
use crate::Determination::{*};

lazy_static! {
    pub static ref RULE: Regex = Regex::new(r"(?:(?<tchar>[xmas])(?<tcmp>[<>])(?<tval>\d+):)?(?<dest>\w+)").unwrap();
    pub static ref WORKFLOW: Regex = Regex::new(r"(?<flow>\w+)\{(?<rules>[^}]+)\}").unwrap();
}

type Part = HashMap<char, u32>;

#[derive(Debug)]
struct Game {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Game {
    fn acceptable_parts(&self) -> Vec<Part> {
        self.parts.iter()
            .filter(|part| {
                let mut wf_name = String::from("in");
                loop {
                    let wf = self.workflows.get(wf_name.as_str()).unwrap();
                    match wf.evaluate(part) {
                        WorkflowLink(name) => wf_name = name,
                        FinalResult(val) => return val
                    }
                }
            })
            .map(|p| p.clone())
            .collect()
    }

    fn discover_rule_flows(&self) -> Vec<(Vec<TestType>, bool)> {
        let mut collector = vec![];
        let empty_path = vec![];
        self.trace_paths("in", &empty_path, &mut collector);

        collector
    }

    fn trace_paths(&self,
                   from_workflow: &str,
                   with_path: &Vec<TestType>,
                   collector: &mut Vec<(Vec<TestType>, bool)>) {
        let rules = &self.workflows.get(from_workflow).unwrap().rules;
        let mut cur_rules = with_path.clone();
        rules.iter().for_each(|rule| {
            match &rule.test {
                // if this "test" always just produces the same result regardless of value,
                // do not add it to the path, just handle it's result
                ALWAYS => {
                    match &rule.determination {
                        // if this always links to the same workflow, trace that workflow
                        WorkflowLink(workflow) => self.trace_paths(workflow.as_str(), &cur_rules, collector),
                        // if this is a terminal result, add the current path to the collector
                        FinalResult(val) => collector.push((cur_rules.clone(), *val))
                    }
                },
                // otherwise this test cares about the value
                gt_lt => {
                    match &rule.determination {
                        FinalResult(val) => {
                            // if passing this test leads us to a conclusion, add this test to
                            // the ruleset, then add the ruleset to the collector
                            let mut final_path = cur_rules.clone();
                            final_path.push(gt_lt.clone());
                            collector.push((final_path, *val));
                        },
                        WorkflowLink(workflow) => {
                            // if passing this test leads us to another workflow, add this test to
                            // the ruleset, then traverse that workflow
                            let mut this_path = cur_rules.clone();
                            this_path.push(gt_lt.clone());
                            self.trace_paths(workflow.as_str(), &this_path, collector);
                        },
                    }
                    // all remaining tests in this workflow are only reached if we fail this test
                    cur_rules.push(invert_test(&rule.test))
                }
            }
        });
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> Determination {
        for rule in self.rules.iter() {
            match rule.test {
                GT(ch, val) => {
                    if *part.get(&ch).unwrap() > val {
                        return rule.determination.clone();
                    }
                    // else continue
                }
                LT(ch, val) => {
                    if *part.get(&ch).unwrap() < val {
                        return rule.determination.clone();
                    }
                    // else continue
                }
                ALWAYS => return rule.determination.clone()
            }
        }
        panic!("Ruleset didn't contain an always value!");
    }
}

#[derive(Debug)]
struct Rule {
    test: TestType,
    determination: Determination,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Determination {
    WorkflowLink(String),
    FinalResult(bool)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone)]
enum TestType {
    GT(char, u32),
    LT(char, u32),
    ALWAYS,
}

fn invert_test(test: &TestType) -> TestType {
    match test {
        GT(ch, val) => LT(*ch, val + 1),
        LT(ch, val) => GT(*ch, val - 1),
        ALWAYS => panic!("Don't try to invert an ALWAYS rule")
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let game = parse_game("input");
    let score: u32 = game.acceptable_parts().iter()
        .flat_map(|p| p.values())
        .sum();
    println!("Part 1: {score}");
}

fn part2() {
    let game = parse_game("input");
    let rule_flows = game.discover_rule_flows();
    let count: usize = rule_flows.iter()
        .filter(|(_, result)| *result == true)
        .map(|(rf, _)| rf)
        .map(size_ruleflow)
        .sum();
    println!("Part 2: {count}");
}

fn size_ruleflow(rules: &Vec<TestType>) -> usize {
    let mut ranges = HashMap::from([
        ('x', 1..4001),
        ('m', 1..4001),
        ('a', 1..4001),
        ('s', 1..4001),
    ]);
    for rule in rules.iter() {
           match rule {
               GT(ch, val) => {
                   let cur_range = ranges.remove(ch).unwrap();
                   let new_begin = max(cur_range.start, val + 1);
                   ranges.insert(*ch, new_begin..cur_range.end);
               },
               LT(ch, val) => {
                   let cur_range = ranges.remove(ch).unwrap();
                   let new_end = min(cur_range.end, *val);
                   ranges.insert(*ch, cur_range.start..new_end);
               },
               ALWAYS => panic!("There shouldn't be any ALWAYS's at this point")
           }
    }

    ranges.values().map(|r| r.size_hint().0).product()
}

fn parse_game(file: &str) -> Game {
    let sections: Vec<String> = fs::read_to_string(file).unwrap()
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();

    let workflows = sections[0].lines()
        .map(parse_workflow)
        .map(|w| (w.name.clone(), w))
        .collect();

    let parts = sections[1].lines()
        .map(parse_part)
        .collect();
    Game { workflows, parts }
}

fn parse_workflow(workflow_str: &str) -> Workflow {
    let line_cap = WORKFLOW.captures(workflow_str).unwrap();
    let name = String::from(line_cap.name("flow").unwrap().as_str());
    let rules: Vec<Rule> = line_cap.name("rules").unwrap()
        .as_str()
        .split(",")
        .map(parse_rule)
        .collect();
    Workflow { name, rules }
}

fn parse_rule(rule_str: &str) -> Rule {
    let rules = RULE.captures(rule_str).unwrap();
    let determination = match rules.name("dest").unwrap().as_str() {
        "A" => FinalResult(true),
        "R" => FinalResult(false),
        other => WorkflowLink(String::from(other))
    };
    return if let Some(test_char_m) = rules.name("tchar") {
        // these is a test-char
        let test_char = test_char_m.as_str().chars().next().unwrap();
        let test_val = rules.name("tval").unwrap().as_str().parse().unwrap();
        let test_type = match rules.name("tcmp").unwrap().as_str() {
            "<" => LT(test_char, test_val),
            ">" => GT(test_char, test_val),
            other => panic!("Unrecognized test type: {other}")
        };
        Rule { test: test_type, determination }
    } else {
        // there is no test-char; this is a static test
        Rule { test: ALWAYS, determination }
    };
}

fn parse_part(part_str: &str) -> Part {
    let trimmed_part_str = &part_str[1..(part_str.len() - 1)];
    trimmed_part_str.split(",")
        .map(|seg| {
            let mut segs = seg.split("=");
            let ch = segs.next().unwrap().chars().next().unwrap();
            let val = segs.next().unwrap().parse().unwrap();
            (ch, val)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Either::{Left, Right};
    use crate::{parse_part, parse_rule, RULE, WORKFLOW};
    use crate::TestType::{*};

    #[test]
    fn regex_test() {
        let found = RULE.captures("a<2006:qkq").unwrap();
        assert_eq!(found.name("tchar").map(|m| m.as_str()), Some("a"));
        assert_eq!(found.name("tcmp").map(|m| m.as_str()), Some("<"));
        assert_eq!(found.name("tval").map(|m| m.as_str()), Some("2006"));
        assert_eq!(found.name("dest").map(|m| m.as_str()), Some("qkq"));


        let found2 = RULE.captures("R").unwrap();
        assert_eq!(found2.name("tchar").map(|m| m.as_str()), None);
        assert_eq!(found2.name("tcmp").map(|m| m.as_str()), None);
        assert_eq!(found2.name("tval").map(|m| m.as_str()), None);
        assert_eq!(found2.name("dest").map(|m| m.as_str()), Some("R"));
    }

    #[test]
    fn workflow_regex() {
        let found = WORKFLOW.captures("px{a<2006:qkq,m>2090:A,rfg}").unwrap();
        assert_eq!(found.name("flow").map(|m| m.as_str()), Some("px"));
        assert_eq!(found.name("rules").map(|m| m.as_str()), Some("a<2006:qkq,m>2090:A,rfg"));
    }

    #[test]
    fn parse_rule_test() {
        let rule1 = parse_rule("a<2006:qkq");
        assert_eq!(rule1.test, LT('a', 2006));
        assert_eq!(rule1.determination, Left(String::from("qkq")));

        let rule2 = parse_rule("m>2090:A");
        assert_eq!(rule2.test, GT('m', 2090));
        assert_eq!(rule2.determination, Right(true));

        let rule3 = parse_rule("rfg");
        assert_eq!(rule3.test, ALWAYS);
        assert_eq!(rule3.determination, Left(String::from("rfg")));
    }

    #[test]
    fn parse_part_test() {
        let part = parse_part("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(part.len(), 4);
        assert_eq!(part.get(&'x'), Some(787).as_ref());
        assert_eq!(part.get(&'m'), Some(2655).as_ref());
        assert_eq!(part.get(&'a'), Some(1222).as_ref());
        assert_eq!(part.get(&'s'), Some(2876).as_ref());
    }
}
