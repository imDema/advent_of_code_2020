use std::collections::HashSet;
use std::hash::Hash;
use std::io::Read;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static!(
    static ref RE_BAG: Regex = Regex::new(r"^(.+) bags contain(.+)$").unwrap();
    static ref RE_BAG_CONTENT: Regex = Regex::new(r"\d+ (\w+ \w+) bag").unwrap();
);

#[derive(Debug)]
struct BagDescription<'a> {
    color: &'a str,
    fs: Vec<&'a str>,
}

impl<'a> BagDescription<'a> {
    pub fn from(s: &'a str) -> Option<Self> {
        let split = RE_BAG.captures(s)?;
        let color =     split.get(1)?.as_str();
        let content =   split.get(2)?.as_str();
        let fs: Vec<&str> = RE_BAG_CONTENT.captures_iter(&content)
            .map(|c| c.get(1).unwrap().as_str()).collect();
        Some(Self{
            color,
            fs,
        })
    }
    pub fn new(s: &'a str) -> Self {
        Self{
            color: s,
            fs: Vec::new(),
        }
    }
    pub fn color(&self) -> &str {&self.color}
    pub fn fs(&self) -> &Vec<&str> {&self.fs}
}

impl Hash for BagDescription<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state)
    }
}
impl PartialEq for BagDescription<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
impl Eq for BagDescription<'_> {}

fn find_bag<'a>(set: &'a HashSet<BagDescription>, root: &'a BagDescription, needle: &str) -> Option<&'a BagDescription<'a>> {
    if root.color() == needle {
        Some(root)
    } else {
        root.fs().iter()
            .filter_map(|b| set.get(&BagDescription::new(b)))
            .find_map(|b| find_bag(set, b, needle))
    }
}

const TARGET: &'static str = "shiny gold";

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    // Build map
    let bags: HashSet<BagDescription> = buf.split('\n')
        .filter_map(BagDescription::from)
        .collect();

    let mut cnt = 0;
    for b in bags.iter() {
        if let Some(_) = find_bag(&bags, b, TARGET) {
            cnt += 1;
            eprintln!("{:?}", b);
        }
    }
    println!("{}", cnt - 1);
}
