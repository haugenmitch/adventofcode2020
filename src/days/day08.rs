#[derive(Debug)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Command {
    inst: Instruction,
    val: i64,
}

#[derive(Debug)]
struct Step {
    line: i64,
    val: i64,
}

pub fn run(part: i32, lines: Vec<String>) {
    let commands = parse_commands(&lines);

    match part {
        1 => part1(&commands),
        2 => part2(&commands),
        _ => {
            part1(&commands);
            part2(&commands);
        }
    }
}

fn parse_commands(lines: &Vec<String>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    for line in lines {
        let tokens: Vec<&str> = line.split(" ").collect();
        let inst: Instruction = match tokens[0] {
            "acc" => Instruction::Acc,
            "jmp" => Instruction::Jmp,
            _ => Instruction::Nop,
        };
        commands.push(Command {
            inst: inst,
            val: tokens[1].parse::<i64>().unwrap(),
        });
    }

    return commands;
}

fn part1(commands: &Vec<Command>) {
    let mut acc: i64 = 0;
    let mut pc: usize = 0;
    let mut inst_exec: Vec<bool> = vec![false; commands.len()];

    loop {
        inst_exec[pc] = true;

        match commands[pc].inst {
            Instruction::Acc => {
                acc += commands[pc].val;
                pc += 1
            }
            Instruction::Jmp => pc = (pc as i64 + commands[pc].val) as usize,
            Instruction::Nop => pc += 1,
        };

        if inst_exec[pc] == true {
            break;
        }
    }

    println!("{}", acc);
}

fn part2(commands: &Vec<Command>) {
    let mut acc: i64 = 0;
    let mut pc: usize = 0;
    let mut inst_exec: Vec<bool> = vec![false; commands.len()];
    let mut swapped: bool = false;
    let mut swap_acc: i64 = 0;
    let mut swap_pc: usize = 0;
    let mut swap_inst_exec: Vec<bool> = vec![];

    loop {
        inst_exec[pc] = true;

        match commands[pc].inst {
            Instruction::Acc => {
                acc += commands[pc].val;
                pc += 1
            }
            Instruction::Jmp => {
                if swapped {
                    pc = (pc as i64 + commands[pc].val) as usize;
                } else {
                    swapped = true;
                    swap_pc = pc;
                    swap_acc = acc;
                    swap_inst_exec = inst_exec.clone();
                    pc += 1;
                }
            }
            Instruction::Nop => {
                if swapped {
                    pc += 1
                } else {
                    swapped = true;
                    swap_pc = pc;
                    swap_acc = acc;
                    swap_inst_exec = inst_exec.clone();
                    pc = (pc as i64 + commands[pc].val) as usize;
                }
            }
        };

        if pc == commands.len() {
            break;
        };

        if inst_exec[pc] == true {
            swapped = false;
            pc = swap_pc;
            acc = swap_acc;
            inst_exec = swap_inst_exec.clone();

            match commands[pc].inst {
                Instruction::Acc => {
                    acc += commands[pc].val;
                    pc += 1
                }
                Instruction::Jmp => pc = (pc as i64 + commands[pc].val) as usize,
                Instruction::Nop => pc += 1,
            };
        }

        if pc == commands.len() {
            break;
        };
    }

    println!("{}", acc);
}
