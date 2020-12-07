use std::io::Read;
use regex::Regex;
use lazy_static::lazy_static;
use petgraph::graphmap::DiGraphMap;

const TARGET: &'static str = "shiny gold";

lazy_static!(
    static ref RE_BAG: Regex = Regex::new(r"^(.+) bags contain(.+)$").unwrap();
    static ref RE_BAG_CONTENT: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
);

struct BagDescription<'a> {
    color: &'a str,
    contents: Vec<(u16, &'a str)>,
}

impl<'a> BagDescription<'a> {
    fn from_str(s: &'a str) -> Result<Self, &'static str> {
        let caps = RE_BAG.captures(s).ok_or_else(|| {dbg!(s); "Invalid structure"})?;
        let head = caps.get(1).ok_or("Missing head")?.as_str();
        let tail = caps.get(2).ok_or("Missing tail")?.as_str();

        let contents: Vec<(u16, &str)> = RE_BAG_CONTENT.captures_iter(tail)
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .map(|(c, col)| (c.parse::<u16>().unwrap(), col))
            .collect();

        Ok(Self{
            color: head,
            contents,
        })
    }
}

fn count_traverse(graph: &DiGraphMap<&str, u16>, node: &str) -> usize {
    let fs_sum: usize = graph.edges(node)
        .map(|(_, to, &e)| e as usize * count_traverse(graph, to))
        .sum();

    1 + fs_sum
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    
    let bags: Vec<BagDescription> = buf.split('\n').filter(|l| l.len() > 0)
        .map(|l| BagDescription::from_str(l).unwrap()).collect();

    let mut graph: DiGraphMap<&str, u16> = DiGraphMap::default();

    for bag in bags.iter() {
        if !graph.contains_node(bag.color) {
            graph.add_node(bag.color);
        }

        for (n, b) in bag.contents.iter() {
            if !graph.contains_node(*b) {
                graph.add_node(*b);
            }
            graph.add_edge(bag.color, *b, *n);
        }
    }

    let cnt = count_traverse(&graph, TARGET);
    println!("{}", cnt - 1);
}
