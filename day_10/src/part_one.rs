use std::error::Error;
use std::io;

pub fn main_p1() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut instructions = Vec::<Instruction>::new();

    // 1. Construct an array with all the instructions
    for line in lines {
        let line = line.unwrap();
        let instr = Instruction::from(&line);

        instructions.push(instr);
    }

    let mut cpu = CPU::new(instructions);
    let mut signal_strength: i32 = 0;

    for cycle in 1..221 {
        if (cycle + 20) % 40 == 0 {
            println!("Signal at cycle {} : {}", cycle, cpu.x());
            signal_strength += cycle * cpu.x();
        }

        cpu.cycle();
    }

    println!("Total signal strength: {}", signal_strength);

    Ok(())
}

struct CPU {
    reg: i32,
    cycles: u32,
    cycle_left: u8,
    instructions: Vec<Instruction>,
    instr_index: usize,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        CPU {
            reg: 1,
            cycles: 0,
            cycle_left: 0,
            instructions,
            instr_index: 0,
        }
    }

    fn cycle(&mut self) {
        self.cycles += 1;

        // 1. Load instruction
        if self.cycle_left == 0 {
            self.cycle_left = self.instructions[self.instr_index].cycles();
        }

        // 3. Do one cycle
        self.cycle_left -= 1;

        // 4. Execute the instruction if it's cycle are done
        if self.cycle_left == 0 {
            match self.instructions[self.instr_index] {
                Instruction::Addx(value) => self.reg += value,
                Instruction::Noop => (),
            };

            // 4.1 Increment instruction pointer
            self.instr_index += 1;
        }
    }

    fn cycles(&self) -> u32 {
        self.cycles
    }

    fn x(&self) -> i32 {
        self.reg
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl From<&String> for Instruction {
    fn from(instr: &String) -> Self {
        let data: Vec<&str> = instr.split(' ').collect();

        match data[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(data[1].parse().unwrap()),
            _ => panic!("Not an instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut instructions = Vec::<Instruction>::new();

        instructions.push(Instruction::Noop);
        instructions.push(Instruction::Addx(3));
        instructions.push(Instruction::Addx(-5));

        let mut cpu = CPU::new(instructions);

        // Noop
        cpu.cycle();
        assert_eq!(cpu.x(), 1);

        // Addx 3
        cpu.cycle();
        assert_eq!(cpu.x(), 1);
        cpu.cycle();
        assert_eq!(cpu.x(), 4);

        // Addx -5
        cpu.cycle();
        assert_eq!(cpu.x(), 4);
        cpu.cycle();
        assert_eq!(cpu.x(), -1);
    }
}
