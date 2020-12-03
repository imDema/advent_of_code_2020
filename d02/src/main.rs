use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct PasswordInfo {
    min: u32,
    max: u32,
    letter: char,
    pass: String,
}

impl PasswordInfo {
    pub fn verify(&self) -> bool {
        let cnt = self.pass.chars().filter(|&c| c == self.letter).count() as u32;
        cnt >= self.min && cnt <= self.max
    }
}

impl FromStr for PasswordInfo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static!(
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        );

        RE.captures(s)
            .and_then(|caps| {
                Some(PasswordInfo{
                    min: caps[1].parse().ok()?,
                    max: caps[2].parse().ok()?,
                    letter: caps[3].chars().nth(0)?,
                    pass: String::from(&caps[4]),
                })
            }).ok_or("Parse error")
    }
}

fn main() {
    let stdin = std::io::stdin();
    let ps =  stdin.lock().lines().filter_map(|l| l.ok())
                .map(|l| l.parse::<PasswordInfo>().unwrap());

    let n = ps.fold(0, |acc, p| acc + p.verify() as usize);
    println!("{}", n);
}
