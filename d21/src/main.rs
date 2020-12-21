use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

type Input<'a> = Vec<(HashSet<&'a str>, HashSet<&'a str>)>;

const NEEDLE: &'static str = " (contains ";

fn parse_input(s: &str) -> Input<'_> {
    let mut v = Vec::new();
    for line in s.split_terminator('\n') {
        let i = line.find(NEEDLE).expect("Invalid format");
        let head = &line[..i];
        let tail = &line[i+NEEDLE.len()..line.len()-1];

        let hs = head.split(' ').collect::<HashSet<_>>();
        let ts = tail.split(", ").collect::<HashSet<_>>();

        v.push((hs,ts))
    }
    v
}

fn choose_compat<'a>(mut compat: HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&'a str, &'a str> {
    let mut to_check: Vec<&str> = compat.iter()
        .filter_map(|(i, v)| if v.len() == 1 {Some(*i)} else {None})
        .collect();

    while to_check.len() > 0 {
        let current = to_check.clone();
        to_check.truncate(0);
        for i in current {
            let needle = *compat[i].iter().next().expect("Error eliminating by domination");
            for (k, v) in compat.iter_mut() {
                if *k != i && v.remove(needle) && v.len() == 1{                    
                    to_check.push(k);
                }
            }
        }
    }
    assert!(compat.iter().all(|v|v.1.len() == 1));
    compat.into_iter().map(|(k,v)| (k, v.into_iter().next().unwrap())).collect()
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = parse_input(&buf);

    let allergens = input.iter().fold(HashSet::new(), |mut acc, (_, all)| {
        all.iter().for_each(|&a| {acc.insert(a);});
        acc
    });

    let map = allergens.into_iter().fold(HashMap::new(), |mut acc, x| {
        let mut iter = input.iter().filter_map(|(ing, all)| if all.contains(x) {Some(ing)} else {None});
        let mut intersection = iter.next().expect("Input too small").clone();
        for al in iter {
            intersection.retain(|e| al.contains(e));
        }
        acc.insert(x, intersection);
        acc
    });

    let compat = choose_compat(map);

    let cnt = input.iter().flat_map(|(l, _)| l.iter())
        .filter(|&&e| compat.values().all(|&a| a != e))
        .count();

    println!("{}", cnt);

    let mut list: Vec<(&str, &str)> = compat.into_iter().collect();
    list.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::from(list[0].1);
    for (_, ing) in list.into_iter().skip(1) {
        out.push(',');
        out.push_str(ing);
    }
    println!("{}", out);
}
