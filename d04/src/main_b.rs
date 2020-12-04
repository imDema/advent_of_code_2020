use std::io::Read;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!(
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref CAP_RE: Regex = Regex::new(r"(\w+):(\S+)").unwrap();
);

#[derive(Debug)]
struct Passport<'a> {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
}

impl<'a> Passport<'a> {
    pub fn new(s: &'a str) -> Option<Passport<'a>> {
        let fields: HashMap<&str, &str> = CAP_RE.captures_iter(s)
            .filter_map(|c| Some((c.get(1)?.as_str(), c.get(2)?.as_str())))
            .collect();

        Some(Passport{
            byr: fields.get("byr")?.parse().ok()?,
            iyr: fields.get("iyr")?.parse().ok()?,
            eyr: fields.get("eyr")?.parse().ok()?,
            hgt: fields.get("hgt")?,
            hcl: fields.get("hcl")?,
            ecl: fields.get("ecl")?,
            pid: fields.get("pid")?,
        })

    }

    pub fn check(&self) -> bool {
        if self.byr < 1920 || self.byr > 2002 {return false}
        if self.iyr < 2010 || self.iyr > 2020 {return false}
        if self.eyr < 2020 || self.eyr > 2030 {return false}
        if let Ok(hval) = self.hgt[..self.hgt.len()-2].parse::<u16>() {
            let hunit = &self.hgt[self.hgt.len()-2..];
            match hunit {
                "cm" => if hval < 150 || hval > 193 {return false},
                "in" => if hval < 59 || hval > 76 {return false},
                _ => return false,
            }
        } else {return false}
        if !HCL_RE.is_match(&self.hcl) {return false}
        if !ECL_RE.is_match(&self.ecl) {return false}
        if !PID_RE.is_match(&self.pid) {return false}
        true
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let res: usize = buf.split("\n\n")
        .filter_map(Passport::new)
        .filter(Passport::check)
        .count();

    println!("{}", res);
}
