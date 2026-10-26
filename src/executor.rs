use crate::parser::{LabeledData, Program};

pub fn execute(mut program: Program) -> Result<(Program, Option<i64>), String> {
    let mut pc = 0usize;

    while pc < program.instructions.len() {
        let instruction_name = program.instructions[pc].mnemonic.name;

        match instruction_name {
            "RUECKGABE" => {
                let return_value = match program.instructions[pc].operands[0].clone() {
                    Some(op) => Functions::rueckgabe(&mut program, [op])?,
                    None => {
                        return Err(format!("RUECKGABE has no matching operand"));
                    }
                };

                return Ok((program, Some(return_value)));
            }
            "ADD" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op1) => match program.instructions[pc].operands[1].clone() {
                        Some(op2) => Functions::add(pc, &mut program, [op1, op2])?,
                        None => {
                            return Err(format!("ADD has no matching second operand"));
                        }
                    },
                    None => {
                        return Err(format!("RUECKGABE has no matching first operand"));
                    }
                };

                pc = new_pc;
            }
            "SUB" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op1) => match program.instructions[pc].operands[1].clone() {
                        Some(op2) => Functions::sub(pc, &mut program, [op1, op2])?,
                        None => {
                            return Err(format!("SUB has no matching second operand"));
                        }
                    },
                    None => {
                        return Err(format!("SUB has no matching first operand"));
                    }
                };

                pc = new_pc;
            }
            "INKREMENT" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op) => Functions::inkrement(pc, &mut program, [op])?,
                    None => {
                        return Err(format!("INKREMENT has no matching operand"));
                    }
                };

                pc = new_pc;
            }
            "DEKREMENT" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op) => Functions::dekrement(pc, &mut program, [op])?,
                    None => {
                        return Err(format!("DEKREMENT has no matching operand"));
                    }
                };

                pc = new_pc;
            }
            "INIT" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op) => Functions::init(pc, &mut program, [op])?,
                    None => {
                        return Err(format!("INIT has no matching operand"));
                    }
                };

                pc = new_pc;
            }
            "SPRUNG" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op) => Functions::sprung(&mut program, [op])?,
                    None => {
                        return Err(format!("SPRUNG has no matching operand"));
                    }
                };

                pc = new_pc;
            }
            "SPRUNG0" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op1) => match program.instructions[pc].operands[1].clone() {
                        Some(op2) => Functions::sprung0(pc, &mut program, [op1, op2])?,
                        None => {
                            return Err(format!("SPRUNG0 has no matching second operand"));
                        }
                    },
                    None => {
                        return Err(format!("SPRUNG0 has no matching first operand"));
                    }
                };

                pc = new_pc;
            }
            "SPRUNGNEG" => {
                let new_pc = match program.instructions[pc].operands[0].clone() {
                    Some(op1) => match program.instructions[pc].operands[1].clone() {
                        Some(op2) => Functions::sprungneg(pc, &mut program, [op1, op2])?,
                        None => {
                            return Err(format!("SPRUNGNEG has no matching second operand"));
                        }
                    },
                    None => {
                        return Err(format!("SPRUNGNEG has no matching first operand"));
                    }
                };

                pc = new_pc;
            }
            _ => {
                return Err(format!("Unrecognized mnemonic '{}'", instruction_name));
            }
        };
    }

    return Ok((program, None));
}

struct Functions;

impl Functions {
    pub fn add(pc: usize, program: &mut Program, operands: [Box<str>; 2]) -> Result<usize, String> {
        let operand2 = match program.data.iter().find(|data| data.label == operands[1]) {
            Some(op) => op,
            None => {
                return Err(format!("{}: Data '{}' not found", "add", operands[0]));
            }
        }
        .data;

        let mut iter = program.data.iter_mut();
        let operand1 = match iter.find(|data| data.label == operands[0]) {
            Some(op) => op,
            None => {
                return Err(format!("{}: Data '{}' not found", "add", operands[0]));
            }
        };

        operand1.data = operand1.data + operand2;

        return Ok(pc + 1);
    }

    pub fn sub(pc: usize, program: &mut Program, operands: [Box<str>; 2]) -> Result<usize, String> {
        let operand2 = match program.data.iter().find(|data| data.label == operands[1]) {
            Some(op) => op,
            None => {
                return Err(format!("{}: Data '{}' not found", "add", operands[0]));
            }
        }
        .data;

        let mut iter = program.data.iter_mut();
        let operand1 = match iter.find(|data| data.label == operands[0]) {
            Some(op) => op,
            None => {
                return Err(format!("{}: Data '{}' not found", "add", operands[0]));
            }
        };

        operand1.data = operand1.data - operand2;

        return Ok(pc + 1);
    }

    pub fn inkrement(
        pc: usize,
        program: &mut Program,
        operands: [Box<str>; 1],
    ) -> Result<usize, String> {
        let operand1 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[0])
        {
            Some(op) => op,
            None => {
                return Err(format!("Data '{}' not found", operands[0]));
            }
        };

        operand1.data = operand1.data + 1;

        return Ok(pc + 1);
    }

    pub fn dekrement(
        pc: usize,
        program: &mut Program,
        operands: [Box<str>; 1],
    ) -> Result<usize, String> {
        let operand1 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[0])
        {
            Some(op) => op,
            None => {
                return Err(format!("Data '{}' not found", operands[0]));
            }
        };

        operand1.data = operand1.data - 1;

        return Ok(pc + 1);
    }

    pub fn init(
        pc: usize,
        program: &mut Program,
        operands: [Box<str>; 1],
    ) -> Result<usize, String> {
        let operand1 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[0])
        {
            Some(op) => op,
            None => {
                program.data.push(LabeledData {
                    label: operands[0].clone(),
                    data: 0,
                });

                return Ok(pc + 1);
            }
        };

        operand1.data = 0;

        return Ok(pc + 1);
    }

    pub fn sprung(program: &mut Program, operands: [Box<str>; 1]) -> Result<usize, String> {
        let i = match program
            .instructions
            .iter()
            .enumerate()
            .find(|(_i, instruction)| {
                instruction
                    .label
                    .as_ref()
                    .is_some_and(|label| *label == operands[0])
            }) {
            Some((i, _instruction)) => i,
            None => {
                return Err(format!(
                    "'{}' is not a valid instruction label",
                    &operands[0]
                ));
            }
        };

        return Ok(i);
    }

    pub fn sprung0(
        pc: usize,
        program: &mut Program,
        operands: [Box<str>; 2],
    ) -> Result<usize, String> {
        let operand2 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[1])
        {
            Some(op) => op,
            None => {
                return Err(format!("Data '{}' not found", operands[1]));
            }
        };

        if operand2.data != 0 {
            return Ok(pc + 1);
        }

        let i = match program
            .instructions
            .iter()
            .enumerate()
            .find(|(_i, instruction)| {
                instruction
                    .label
                    .as_ref()
                    .is_some_and(|label| *label == operands[0])
            }) {
            Some((i, _instruction)) => i,
            None => {
                return Err(format!(
                    "'{}' is not a valid instruction label",
                    &operands[0]
                ));
            }
        };

        return Ok(i);
    }

    pub fn sprungneg(
        pc: usize,
        program: &mut Program,
        operands: [Box<str>; 2],
    ) -> Result<usize, String> {
        let operand2 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[1])
        {
            Some(op) => op,
            None => {
                return Err(format!("Data '{}' not found", operands[1]));
            }
        };

        if operand2.data >= 0 {
            return Ok(pc + 1);
        }

        let i = match program
            .instructions
            .iter()
            .enumerate()
            .find(|(_i, instruction)| {
                instruction
                    .label
                    .as_ref()
                    .is_some_and(|label| *label == operands[0])
            }) {
            Some((i, _instruction)) => i,
            None => {
                return Err(format!(
                    "'{}' is not a valid instruction label",
                    &operands[0]
                ));
            }
        };

        return Ok(i);
    }

    pub fn rueckgabe(program: &mut Program, operands: [Box<str>; 1]) -> Result<i64, String> {
        let operand1 = match program
            .data
            .iter_mut()
            .find(|data| data.label == operands[0])
        {
            Some(op) => op,
            None => {
                return Err(format!("{}: Data '{}' not found", "rueckgabe", operands[0]));
            }
        };

        return Ok(operand1.data);
    }
}
