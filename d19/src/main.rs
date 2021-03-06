use std::collections::HashMap;
use std::io::{stdin, Read};
use std::iter::{empty, once};

#[derive(Debug)]
enum Rule {
    Simple(String),
    Ref(Vec<Vec<usize>>),
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let (n, rest) = s.split_at(s.find(": ").expect("Invalid format"));
    let id: usize = n.parse().expect("Invalid id format");
    let rule = if rest.contains("\"") {
        let content = rest.split("\"").nth(1).expect("Invalid literal");
        Rule::Simple(content.to_owned())
    } else {
        let mut or = Vec::new();
        let mut and = Vec::new();
        for tok in rest[2..].split_whitespace() {
            match tok {
                "|" => {
                    or.push(and); and = Vec::new()
                },
                _ => {
                    let num: usize = tok.parse().expect("Cannot parse rule num");
                    and.push(num);
                }
            }
        }
        or.push(and);
        Rule::Ref(or)
    };
    (id, rule)
}

type RuleMap = HashMap<usize, Rule>;
fn parse_rules(s: &str) -> Result<RuleMap, &'static str> {
    let mut map = HashMap::new();
    for l in s.split_terminator("\n") {
        let (id, rule) = parse_rule(l);
        map.insert(id, rule);
    }
    Ok(map)
}

fn eval_seq<'a>(rules: &'a RuleMap, and: &'a [usize], input: &'a str) -> Box<dyn Iterator<Item=&'a str> + 'a> {
    match and {
        [x] => eval_rule(rules, *x, input),
        v => {
            let h = eval_rule(rules, v[0], input);
            Box::new(h.into_iter().flat_map(move |head| {
                let next_match = eval_seq(rules, &and[1..], &input[head.len()..]);
                next_match.into_iter()
                    .map(move |m| &input[..head.len()+m.len()])
            }))
        }
    }
}

fn eval_rule<'a>(rules: &'a RuleMap, id: usize, input: &'a str) -> Box<dyn Iterator<Item=&'a str> + 'a> {
    let r = rules.get(&id).expect("Missing rule");
    match r {
        Rule::Simple(pat) => {
            if input.starts_with(pat) {
                Box::new(once(&input[..pat.len()]))
            } else {
                Box::from(empty())
            }
        },
        Rule::Ref(v) => {
            Box::new(v.into_iter()
                .flat_map(move |or| eval_seq(rules, &or, input)))
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (rules, strings) = buf.split_at(buf.find("\n\n").unwrap());
    let strings = &strings[2..];

    let mut rules = parse_rules(rules).expect("Could not generate map");

    // Part one
    let cnt = strings.split_terminator("\n")
        .map(|s| (s, eval_rule(&rules, 0, s)))
        .filter_map(|(s, mut vs)| vs.find(|&v| v == s))
        .count();

    println!("{}", cnt);

    // Part two
    rules.insert(8, Rule::Ref(vec![vec![42],vec![42, 8]]));
    rules.insert(11, Rule::Ref(vec![vec![42, 31], vec![42,11,31]]));

    let cnt = strings.split_terminator("\n")
        .map(|s| (s, eval_rule(&rules, 0, s)))
        .filter_map(|(s, mut vs)| vs.find(|&v| v == s))
        .count();

    println!("{}", cnt);
}
