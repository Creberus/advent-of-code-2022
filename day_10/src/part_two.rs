use std::error::Error;
use std::io;

pub fn main_p2() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();

    let mut instructions = Vec::<Instruction>::new();
    let mut crt = [[char::default(); 40]; 6];

    // 1. Construct an array with all the instructions
    for line in lines {
        let line = line.unwrap();
        let instr = Instruction::from(&line);

        instructions.push(instr);
    }

    let mut cpu = CPU::new(instructions);
    let mut row: usize = 0;

    for cycle in 1..240 {
        // 1. Init cycle
        let position = (cycle - 1) % 40 as usize;
        if cycle % 40 == 0 {
            row += 1;
        }

        // 2. During cycle
        let sprite_pos = cpu.x();

        println!("Sprite position: {}", sprite_pos);

        if position as i32 >= sprite_pos - 1 && position as i32 <= sprite_pos + 1 {
            crt[row][position] = '#'
        } else {
            crt[row][position] = '.'
        }

        // 3. Execute cycle
        cpu.cycle();
    }

    for row in crt {
        row.into_iter().for_each(|c| print!("{}", c));
        println!("");
    }

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
