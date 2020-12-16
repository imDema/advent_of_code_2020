use std::io::{stdin, Read};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE_RULE : Regex = Regex::new(r": (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
);

struct Rule {
    a: (u16,u16),
    b: (u16,u16),
}

impl Rule {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        let caps = RE_RULE.captures(s).ok_or("Invalid format")?;
        let c: Vec<u16> = caps.iter().skip(1).filter_map(|cap| cap)
            .filter_map(|cap| cap.as_str().parse().ok()).collect();

        assert!(c.len() == 4);
        let a = (c[0], c[1]);
        let b = (c[2], c[3]);
        Ok(Self{a, b})
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
        l.split(',').map(|n| n.parse().unwrap()).collect::<Vec<u16>>()
    })
}

fn check_tickets<I: Iterator<Item=Vec<u16>>>(rules: &Vec<Rule>, tickets: I) {
    let mut sum: u64 = 0;
    for t in tickets {
        sum += t.iter().filter(|&&n| rules.iter().all(|r| !r.check(n)))
            .map(|&x| x as u64).sum::<u64>();
    }
    println!("{}", sum);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split("\n\n");

    let rules = parse_rules(parts.next().unwrap());

    let tickets = parse_tickets(parts.nth(1).unwrap());

    check_tickets(&rules, tickets);
}
