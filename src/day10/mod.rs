use crate::day::{Answer, Day};

pub struct Day10;
impl Day for Day10 {
    type TypePart1 = i32;
    type TypePart2 = String;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let input = self.get_input_for_day_by_line(10);

        let mut instructions = Vec::new();
        for instruction in input {
            if instruction.starts_with("addx") {
                instructions.push(
                    Instruction::Addx(instruction[5..].parse::<i32>().unwrap())
                );
            } else if instruction.starts_with("noop") {
                instructions.push(Instruction::Noop);
            }
        }

        let mut cpu = Cpu::new(instructions);
        let mut part_1 = 0;
        let mut part_2: Vec<char> = Vec::new();
        part_2.resize(240, '.');

        for cycle in 1usize..=240 {
            if (cycle + 20) % 40 == 0 {
                part_1 += cpu.register_x * cycle as i32;
            }
            let crt_position = ((cycle - 1) % 40) as i32;
            if cpu.register_x - 1 <= crt_position && crt_position <= cpu.register_x + 1 {
                part_2[cycle-1] = '#';
            }
            if !cpu.step() {
                break;
            }
        }

        let part_2: Vec<u8> = part_2.iter().map(|elem| *elem as u8).collect();
        let mut part_2_printable = String::from("\n");
        for i in 0usize..6 {
            let index = i * 40;
            part_2_printable += &format!(
                "{}\n",
                String::from_utf8(part_2[index..index+40].to_vec()).unwrap()
            );
        }

        Answer::new(Some(part_1), Some(part_2_printable))
    }
}
impl Day10 {
    pub fn new() -> Day10 {
        Day10
    }
}

struct Cpu {
    instructions: Vec<Instruction>,
    program_counter: usize,
    current_instruction_cycles: u32,

    register_x: i32,
}
impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Cpu {
        let current_instruction_cycles = instructions[0].execution_time();
        Cpu {
            instructions,
            program_counter: 0,
            current_instruction_cycles,
            register_x: 1,
        }
    }

    fn step(&mut self) -> bool {
        self.current_instruction_cycles -= 1;
        if self.current_instruction_cycles == 0 {
            // dispatch the current instruction
            match self.instructions[self.program_counter] {
                Instruction::Addx(V) => self.register_x += V,
                Instruction::Noop => {},
            }
            self.program_counter += 1;
            self.current_instruction_cycles
                = match self.instructions.get(self.program_counter)
            {
                Some(instruction) => instruction.execution_time(),
                None => return false,
            };
        }
        return true;
    }
}

enum Instruction {
    Addx(i32),
    Noop,
}
impl Instruction {
    fn execution_time(&self) -> u32 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}