use std::io::BufRead;

fn parse_id(s: String) -> Option<u16> {
    let code = s.chars().map(|c| match c {
        'F' | 'L' => '0',
        'B' | 'R' => '1',
        oth => oth,
    }).collect::<String>();

    u16::from_str_radix(&code, 2).ok()
} 

fn main() {
    let stdin = std::io::stdin();
    let ids = stdin.lock().lines().filter_map(|l| l.ok())
        .filter_map(parse_id);

    let max = ids.max().unwrap();

    println!("{}", max);
}
