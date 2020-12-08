use std::str::FromStr;
use std::io::BufRead;

enum Op {
    Acc(i16),
    Jmp(i16),
    Nop,
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        let opcode = splits.next().ok_or("Not enough tokens, 0 supplied, 2 required")?;
        let imm_str = splits.next().ok_or("Not enough tokens, 1 supplied, 2 required")?;
        let imm = i16::from_str_radix(&imm_str, 10).map_err(|_| "Cannot parse immediate")?;

        match opcode {
            "acc" => Ok(Op::Acc(imm)),
            "jmp" => Ok(Op::Jmp(imm)),
            "nop" => Ok(Op::Nop),
            _ => return Err("Invalid opcode"),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let prog: Vec<Op> = stdin.lock().lines().filter_map(|l|l.ok())
        .map(|l| Op::from_str(&l).unwrap()).collect();

    let mut prog_marked: Vec<(Op, bool)> = prog.into_iter().map(|op| (op, false)).collect();
    let mut pc: isize = 0;
    let mut acc = 0;
    while let Some((op, mark)) = prog_marked.get_mut(pc as usize) {
        if *mark {
            println!("acc: {}", acc);
            break;
        }
        *mark = true;
        match *op {
            Op::Acc(imm) => acc += imm,
            Op::Jmp(imm) => pc += (imm - 1) as isize,
            Op::Nop => (),
        }
        pc += 1;
    }
}
