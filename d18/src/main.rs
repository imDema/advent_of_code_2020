use pest::{Parser, iterators::Pair};
use std::io::{stdin, Read};

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExpressionParser;

fn eval(pair: Pair<'_, Rule>) -> usize {
    match pair.as_rule() {
        Rule::expr => {
            let mut pairs = pair.into_inner();
            let mut res = eval(pairs.next().unwrap());
            while let (Some(op), Some(term)) = (pairs.next(), pairs.next()) {
                match op.as_rule() {
                    Rule::add => res += eval(term),
                    Rule::mult => res *= eval(term),
                    _ => unreachable!("Invalid path in AST"),
                }
            }
            res
        },
        Rule::num => {
            let literal = pair.as_str();
            usize::from_str_radix(literal, 10).unwrap()
        },
        _ => unreachable!("Invalid path in AST"),
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut sum = 0;
    for l in buf.split_terminator('\n') {
        let parsed = ExpressionParser::parse(Rule::calculation, l).expect("Unable to parse")
            .next().unwrap();
        let val = eval(parsed);
        sum += val;
    }
    println!("{}", sum);
}
