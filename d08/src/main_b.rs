use std::str::FromStr;
use std::io::BufRead;

#[derive(Copy, Clone)]
enum Op {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
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
            "nop" => Ok(Op::Nop(imm)),
            _ => return Err("Invalid opcode"),
        }
    }
}
#[derive(Copy, Clone)]
enum ProgramStatus {
    Running,
    Completed,
    NonTerminating,
    Segfault,
}

#[derive(Copy, Clone)]
struct Registry{pc: isize, acc: isize}

#[derive(Clone)]
struct Processor {
    program: Vec<(Op, bool)>,
    reg: Registry,
    status: ProgramStatus,
}

impl Processor {
    pub fn new (program: Vec<Op>, pc: isize, acc: isize) -> Self {
        let program: Vec<(Op, bool)> = program.into_iter().map(|op| (op, false)).collect();
        Self {
            program,
            reg: Registry{pc, acc},
            status: ProgramStatus::Running,
        }
    }

    pub fn run_all (&mut self) {
        while let Some(_) = self.next() {}
    }

    pub fn get_program(&mut self) -> &Vec<(Op, bool)> { &self.program }

    pub fn get_program_mut(&mut self) -> &mut Vec<(Op, bool)> { &mut self.program }

    pub fn get_reg(&self) -> Registry { self.reg }

    pub fn get_status(&self) -> ProgramStatus { self.status }
}


impl Iterator for Processor {
    type Item = Registry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((op, mark)) = self.program.get_mut(self.reg.pc as usize) {
            if *mark {
                self.status = ProgramStatus::NonTerminating;
                return None
            }
            *mark = true;
            match *op {
                Op::Acc(imm) => self.reg.acc += imm as isize,
                Op::Jmp(imm) => self.reg.pc += (imm - 1) as isize,
                Op::Nop(_) => (),
            }
            self.reg.pc += 1;
            if self.reg.pc as usize == self.program.len() {
                self.status = ProgramStatus::Completed;
                return None;
            }
        } else {
            self.status = ProgramStatus::Segfault;
            return None;
        }
        Some(self.reg)
    }
}

fn swap_jmp_nop(prog: &mut Vec<(Op, bool)>,  addr: usize) {
    match prog[addr].0 {
        Op::Jmp(imm) => {
            prog[addr].0 = Op::Nop(imm);
        }
        Op::Nop(imm) => {
            prog[addr].0 = Op::Nop(imm);
        }
        _ => {},
    }
}

fn main() {
    let stdin = std::io::stdin();
    let prog: Vec<Op> = stdin.lock().lines().filter_map(|l|l.ok())
        .map(|l| Op::from_str(&l).unwrap()).collect();

    let mut main = Processor::new(prog, 0, 0);

    while let Some(reg) = main.next() {
        let addr = reg.pc as usize;
        let (op, _) = main.get_program()[addr];
        if let Op::Acc(_) = op {
            continue;
        } else {
            let mut fork = main.clone();
            swap_jmp_nop(fork.get_program_mut(), addr);
            
            fork.run_all();
            if let ProgramStatus::Completed = fork.get_status() {
                println!("Terminated, acc: {}", fork.get_reg().acc);
                break;
            }
        }
    }   
}
