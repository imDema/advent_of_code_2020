use std::io::Read;

fn parse_group(g: &str) -> u32 {
    g.chars()
        .filter(|&c| c != '\n')
        .fold(0u32, |acc, x| acc | (1 << (x as u8 - 'a' as u8)))
        .count_ones()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let res: u32 = buf.split("\n\n")
        .map(parse_group)
        .sum();

    println!("{}", res);
}
