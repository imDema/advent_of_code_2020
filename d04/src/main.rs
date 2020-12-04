use std::io::Read;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid):").unwrap(); 
    );

    let res: usize = buf.split("\n\n")
        .filter(|s| RE.find_iter(s).count() == 7)
        .count();

    println!("{}", res);
}
