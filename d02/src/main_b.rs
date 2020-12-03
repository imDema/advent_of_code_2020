use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct PasswordInfo {
    fst: usize,
    snd: usize,
    letter: char,
    pass: String,
}

impl PasswordInfo {
    pub fn verify(&self) -> bool {
        let chars = self.pass.as_bytes();
        let (fst, snd) = (chars[self.fst - 1] as char, chars[self.snd - 1] as char);
        (fst == self.letter) != (snd == self.letter)
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
                    fst: caps[1].parse().ok()?,
                    snd: caps[2].parse().ok()?,
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
