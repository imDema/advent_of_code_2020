use std::io::{stdin, Read};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE_RULE : Regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
);

#[derive(Clone, Debug)]
struct Rule {
    field: String,
    a: (u16,u16),
    b: (u16,u16),
}

impl Rule {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        let caps = RE_RULE.captures(s).ok_or("Invalid format")?;
        let c: Vec<&str> = caps.iter()
            .skip(1)
            .filter_map(|cap| Some(cap?.as_str()))
            .collect();

        assert!(c.len() == 5);
        let field = c[0].to_owned();
        let a = (c[1].parse().unwrap(), c[2].parse().unwrap());
        let b = (c[3].parse().unwrap(), c[4].parse().unwrap());
        Ok(Self{field, a, b})
    }

    #[inline]
    pub fn check(&self, x: u16) -> bool {
        self.a.0 <= x && x <= self.a.1 || self.b.0 <= x && x <= self.b.1
    }
}

fn parse_rules(s: &str) -> Vec<Rule> {
    s.split('\n').map(|s| Rule::new(s).unwrap()).collect()
}

fn parse_tickets(s: &str) -> impl Iterator<Item=Vec<u16>> + '_ {
    let lines = s.split('\n').skip(1).filter(|l| l.len() > 1);
    lines.map(|l| {
        l.split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>()
    })
}

fn collect_valid_tickets<I: Iterator<Item=Vec<u16>>>(rules: &Vec<Rule>, tickets: I) -> Vec<Vec<u16>> {
    let mut tk = Vec::new();

    for t in tickets {
        if t.iter().all(|&n| rules.iter().any(|r| r.check(n))) {
            tk.push(t);
        }
    }
    tk
}

// Panics if compatibility cannot resolve through domination
fn choose_compat(mut compat: Vec<Vec<usize>>) -> Vec<usize> {
    let mut to_check: Vec<usize> = compat.iter()
        .enumerate()
        .filter_map(|(i, v)| if v.len() == 1 {Some(i)} else {None})
        .collect();

    while to_check.len() > 0 {
        let current = to_check.clone();
        to_check.truncate(0);
        for i in current {
            let needle = compat[i][0];
            for j in 0..compat.len() {
                if i == j {continue;}
                if let Some(pos) = compat[j].iter().position(|x| *x == needle) {
                    compat[j].swap_remove(pos);
                    
                    if compat[j].len() == 1 {
                        to_check.push(j);
                    }
                }
            }
        }
    }
    assert!(compat.iter().all(|v|v.len() == 1));
    compat.into_iter().map(|v| v[0]).collect()
}

// This could be improved
fn permutate<T: Clone>(vec: &mut Vec<T>, order: &Vec<usize>) {
    assert!(vec.len() == order.len());
    let len = order.len();
    for &i in order {
        vec.push(vec[i].clone());
    }
    for i in 0..len {
        vec.swap(i, i+len);
    }
    vec.truncate(len);
    vec.shrink_to_fit();
}

fn sort_rules(rules: &mut Vec<Rule>, tickets: Vec<Vec<u16>>) {
    let compatibility = (0..rules.len()).fold(Vec::new(), |mut vec, i| {
        let valid: Vec<usize> = rules.iter()
            .enumerate()
            .filter(|r| tickets.iter().all(|t| r.1.check(t[i])))
            .map(|r|r.0)
            .collect();
        vec.push(valid);
        vec
    });
    let order = choose_compat(compatibility);
    permutate(rules, &order);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split("\n\n");

    let mut rules = parse_rules(parts.next().unwrap());

    let my_ticket = parse_tickets(parts.next().unwrap()).nth(0).unwrap();

    let tickets: Vec<Vec<u16>> = collect_valid_tickets(&rules, parse_tickets(parts.nth(0).unwrap()));

    sort_rules(&mut rules, tickets);

    let idxes = rules.iter()
        .enumerate()
        .filter(|&r| r.1.field.starts_with("departure"))
        .map(|r|r.0).collect::<Vec<usize>>();
    
    let prod: usize = idxes.into_iter().map(|i| my_ticket[i] as usize).product();
    println!("{}", prod);
}
