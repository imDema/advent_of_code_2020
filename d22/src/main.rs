use std::collections::VecDeque;
use std::io::{stdin, Read};
use std::collections::HashSet;


type Input = (VecDeque<u8>, VecDeque<u8>);

enum Winner {
    P0,
    P1,
}

fn parse_input(s: &str) -> Input {
    let needle = "\n\n";
    let x = s.find(needle).expect("Invalid format");
    let p = (&s[..x], &s[x+needle.len()..]);

    let d0 = p.0.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    let d1 = p.1.lines().skip(1).map(|l| l.parse().unwrap()).collect();

    (d0, d1)
}

fn calc_score(deck: VecDeque<u8>) -> usize {
    deck.iter().rev().enumerate()
        .fold(0, |acc, (i,&x)| acc + (i+1)*(x as usize))
}

fn push_cards(decks: &mut Input, winner: Winner) {
    let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
    match winner {
        Winner::P0 => {
            decks.0.push_back(cards.0);
            decks.0.push_back(cards.1);
        }
        Winner::P1 => {
            decks.1.push_back(cards.1);
            decks.1.push_back(cards.0);
        }
    }
}

fn sym(decks: &mut Input) -> Winner {
    loop {
        if decks.0.len() > 0 && decks.1.len() > 0 {
            let c = (*decks.0.front().unwrap(), *decks.1.front().unwrap());
            let round_winner = if c.0 > c.1 {
                Winner::P0
            } else {
                Winner::P1
            };
            push_cards(decks, round_winner);

        } else if decks.0.len() > 0 {
            break Winner::P0
        } else {
            break Winner::P1
        }
    }
}

fn sym_rec(decks: &mut Input) -> Winner {
    let mut history = HashSet::new();

    loop {
        if !history.insert(decks.clone()) {
            break Winner::P0;
        }
        if decks.0.len() > 0 && decks.1.len() > 0 {
            let c = (*decks.0.front().unwrap(), *decks.1.front().unwrap());
        
            let round_winner = if (c.0 as usize) < decks.0.len() && (c.1 as usize) < decks.1.len() {
                let mut subdecks = (
                    decks.0.iter().map(|v| *v).skip(1).take(c.0 as usize).collect(),
                    decks.1.iter().map(|v| *v).skip(1).take(c.1 as usize).collect(),
                );
                sym_rec(&mut subdecks)

            } else if c.0 > c.1 {
                Winner::P0
            } else {
                Winner::P1
            };
            push_cards(decks, round_winner);

        } else if decks.0.len() > 0 {
            break Winner::P0
        } else {
            break Winner::P1
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut decks = parse_input(&buf);

    let score = match sym(&mut decks) {
        Winner::P0 => calc_score(decks.0),
        Winner::P1 => calc_score(decks.1),
    };

    println!("{}", score);

    let mut decks = parse_input(&buf);

    let score = match sym_rec(&mut decks) {
        Winner::P0 => calc_score(decks.0),
        Winner::P1 => calc_score(decks.1),
    };

    println!("{}", score);
}
