#![feature(iter_array_chunks)]
use std::fmt::Display;

use itertools::Itertools;

advent_of_code::solution!(17);

type ComboOperant = u8;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(ComboOperant),
    Bxl(u8),
    Bst(ComboOperant),
    Jnz(u8),
    Bxc(u8),
    Out(ComboOperant),
    Bdv(ComboOperant),
    Cdv(ComboOperant),
}

impl From<[u8; 2]> for Instruction {
    fn from(value: [u8; 2]) -> Self {
        match value[0] {
            0 => Self::Adv(value[1]),
            1 => Self::Bxl(value[1]),
            2 => Self::Bst(value[1]),
            3 => Self::Jnz(value[1]),
            4 => Self::Bxc(value[1]),
            5 => Self::Out(value[1]),
            6 => Self::Bdv(value[1]),
            7 => Self::Cdv(value[1]),
            _ => unreachable!("unknown opcode reached"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Instruction {
    fn as_u8(&self) -> [u8; 2] {
        match self {
            Instruction::Adv(x) => [0, *x],
            Instruction::Bxl(x) => [1, *x],
            Instruction::Bst(x) => [2, *x],
            Instruction::Jnz(x) => [3, *x],
            Instruction::Bxc(x) => [4, *x],
            Instruction::Out(x) => [5, *x],
            Instruction::Bdv(x) => [6, *x],
            Instruction::Cdv(x) => [7, *x],
        }
    }
}

#[derive(Debug, Default)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    output_buffer: Vec<usize>,
    code: Vec<Instruction>,
}

impl From<&str> for Cpu {
    fn from(value: &str) -> Self {
        let mut cpu = Self::default();
        for l in value.trim().lines() {
            if l.starts_with("Register") {
                match l.chars().nth(9) {
                    Some('A') => cpu.a = l.get(12..).unwrap().parse::<usize>().unwrap(),
                    Some('B') => cpu.b = l.get(12..).unwrap().parse::<usize>().unwrap(),
                    Some('C') => cpu.c = l.get(12..).unwrap().parse::<usize>().unwrap(),
                    _ => unreachable!("Unknown register initialized"),
                }
            } else if l.starts_with("Program") {
                let program_str = l.get(9..).expect("program string invalid");
                let program = program_str
                    .split(',')
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect_vec();
                cpu.code = program
                    .into_iter()
                    .array_chunks()
                    .map(Instruction::from)
                    .collect_vec();
            }
        }
        cpu
    }
}

impl Cpu {
    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.pc = 0;
        self.output_buffer = vec![];
    }

    fn peek_instruction(&self) -> Option<&Instruction> {
        self.code.get(self.pc)
    }

    fn step(&mut self) -> bool {
        let instruction = match self.peek_instruction() {
            Some(&i) => i,
            None => return false,
        };

        self.execute_instruction(instruction);

        true
    }

    fn read_combo_operant(&self, value: ComboOperant) -> usize {
        match value {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("reserved combo operand value"),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Adv(c) => {
                self.a /= 2_usize.pow(self.read_combo_operant(c) as u32);
            }
            Instruction::Bxl(x) => {
                self.b ^= x as usize;
            }
            Instruction::Bst(c) => {
                self.b = self.read_combo_operant(c) % 8;
            }
            Instruction::Jnz(x) => {
                if self.a != 0 {
                    self.pc = (x / 2) as usize;
                    return;
                }
            }
            Instruction::Bxc(_) => self.b ^= self.c,
            Instruction::Out(c) => {
                self.output_buffer.push(self.read_combo_operant(c) % 8);
            }
            Instruction::Bdv(c) => {
                self.b = self.a / 2_usize.pow(self.read_combo_operant(c) as u32);
            }
            Instruction::Cdv(c) => {
                self.c = self.a / 2_usize.pow(self.read_combo_operant(c) as u32);
            }
        }
        self.pc += 1;
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn output_str(&self) -> String {
        self.output_buffer.iter().map(|c| c.to_string()).join(",")
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CPU: PC {} {:?}", self.pc, self.peek_instruction())?;
        writeln!(f, "     A  {:032b} {}", self.a, self.a)?;
        writeln!(f, "     B  {:032b} {}", self.b, self.b)?;
        writeln!(f, "     C  {:032b} {}", self.c, self.c)?;
        writeln!(f, "     Code {:?}", self.code)?;
        writeln!(f, "     Output {:?}", self.output_buffer)
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cpu = Cpu::from(input);
    cpu.run();
    Some(cpu.output_str())
}

pub fn part_two(input: &str) -> Option<usize> {
    // This solution is highly dependent on my input string
    // It might not work for other inputs.
    // TODO: check if there is a way to reverse all instructions.
    //       probably not, because we cannot guess the register
    //       contents.

    // Program in text:
    // 0. Store last 3 bits of A into B
    // 1. Invert last bit of B
    // 2. Store into C the result of A shifted by B
    // 3. Shift A by 3 bit
    // 4. XOR B and C
    // 5. Invert first two bits of B
    // 6. Output B
    // 7. Loop if A has bits left

    // A must be 3 * #instructions bits long (because for each OUT instruction, we rightshift A by 3)
    // This means we can recursively search for A by finding three bits that
    // would output the last instruction, then use those bits as prefix to find
    // the next three bits that would output the last two instructions, use
    // those 6 bits as prefix to search the next 3, etc, until we find the
    // full bitstring.

    let mut cpu = Cpu::from(input);
    let program = cpu.code.iter().flat_map(|i| i.as_u8()).rev().collect_vec();
    let mut stack = vec![0];

    loop {
        let to_match = program.get(0..stack.len()).unwrap();
        let mut reconstructed_a = 0;
        for i in stack.iter() {
            reconstructed_a <<= 3;
            reconstructed_a |= i;
        }
        cpu.reset();
        cpu.a = reconstructed_a;
        cpu.run();
        if cpu
            .output_buffer
            .iter()
            .rev()
            .map(|e| *e as u8)
            .collect::<Vec<u8>>()
            == to_match
        {
            if stack.len() == program.len() {
                break;
            }
            stack.push(0);
        } else {
            let mut last_iteration = stack.pop().unwrap();
            if last_iteration >= 7 {
                last_iteration = stack.pop().unwrap();
            }
            stack.push(last_iteration + 1);
        }
    }

    // test it
    let program_code = program.iter().rev().join(",");
    let mut reconstructed_a = 0;
    for i in stack {
        reconstructed_a <<= 3;
        reconstructed_a |= i;
    }
    cpu.reset();
    cpu.a = reconstructed_a;
    cpu.run();
    assert_eq!(program_code, cpu.output_str());

    Some(reconstructed_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(247839002892474));
    }
}
