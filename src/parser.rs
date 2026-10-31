use crate::tokenizer::{Mnemonic, Token};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mnemonic2 {
    pub name: &'static str,
    pub operands: Option<(Operand2, Option<Operand2>)>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operand2 {
    CodeLabel,
    DataLabel,
}

#[derive(Debug, PartialEq)]
pub struct Program2 {
    pub instructions: Vec<Instruction2>,
    pub data: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction2 {
    ADD(usize, usize),
    SUB(usize, usize),
    INKREMENT(usize),
    DEKREMENT(usize),
    INIT(usize),
    RUECKGABE(usize),
    SPRUNG(usize),
    SPRUNG0(usize, usize),
    SPRUNGNEG(usize, usize),
}

// TODO: Rewrite labels to use indexes inside the programs arrays instead of string names
// change `Program` to use fixed-size arrays instead of vecs since all operands that will be used
// are known ahead of time; This should reduce the time complexity of label lookups from O(n) to O(1)
// and reduce the constant coefficient due to dropping the comparison
#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub data: Vec<LabeledData>,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub label: Option<Box<str>>,
    pub mnemonic: Mnemonic,
    pub operands: [Option<Box<str>>; 2],
}

#[derive(Debug, PartialEq)]
pub struct LabeledData {
    pub label: Box<str>,
    pub data: i64,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut program = Program {
        instructions: vec![],
        data: vec![],
    };

    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Label(label) => match iter.peek() {
                Some(Token::Label(label2)) => {
                    return Err(format!(
                        "Error parsing {}: Expected data or mnemonic but got '{}'",
                        label, label2
                    ));
                }
                Some(Token::Data(data)) => {
                    let parsed_data = match data.parse() {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            return Err(format!(
                                "Error parsing data: {} is not valid data! {e}",
                                data
                            ));
                        }
                    };

                    if let Some(_data) = program.data.iter().find(|data| data.label == *label) {
                        return Err(format!("Redefinition of data '{label}'"));
                    }

                    program.data.push(LabeledData {
                        label: label.clone(),
                        data: parsed_data,
                    });

                    iter.next();
                }
                Some(Token::Mnemonic(mnemonic)) => {
                    if let Some(_instruction) = program.instructions.iter().find(|instruction| {
                        instruction
                            .label
                            .as_ref()
                            .is_some_and(|instruction_label| *instruction_label == *label)
                    }) {
                        return Err(format!("Redefinition of code label '{label}'"));
                    }

                    iter.next();
                    if let Some(_) = mnemonic.operands[0] {
                        let operand1 = match iter.peek() {
                            Some(Token::Label(operand)) => operand,
                            None => {
                                return Err("Expected an operand but got EOF".to_string());
                            }
                            Some(Token::Mnemonic(mnemonic)) => {
                                return Err(format!(
                                    "Expected an operand but got mnemonic '{}'",
                                    mnemonic.name
                                ));
                            }
                            Some(Token::Data(data)) => {
                                return Err(format!("Expected an operand but got data '{}'", data));
                            }
                        };

                        iter.next();

                        if let Some(_) = mnemonic.operands[1] {
                            let operand2 = match iter.peek() {
                                Some(Token::Label(operand)) => operand,
                                None => {
                                    return Err("Expected an operand but got EOF".to_string());
                                }
                                Some(Token::Mnemonic(mnemonic)) => {
                                    return Err(format!(
                                        "Expected an operand but got mnemonic '{}'",
                                        mnemonic.name
                                    ));
                                }
                                Some(Token::Data(data)) => {
                                    return Err(format!(
                                        "Expected an operand but got data '{}'",
                                        data
                                    ));
                                }
                            };

                            iter.next();

                            program.instructions.push(Instruction {
                                label: Some(label.clone()),
                                mnemonic: *mnemonic,
                                operands: [Some(operand1.clone()), Some(operand2.clone())],
                            });
                        } else {
                            program.instructions.push(Instruction {
                                label: Some(label.clone()),
                                mnemonic: *mnemonic,
                                operands: [Some(operand1.clone()), None],
                            });
                        }
                    } else {
                        program.instructions.push(Instruction {
                            label: Some(label.clone()),
                            mnemonic: *mnemonic,
                            operands: [None, None],
                        });
                    }
                }
                None => return Err("Unexpected end of input".to_string()),
            },
            Token::Mnemonic(mnemonic) => {
                if let Some(_) = mnemonic.operands[0] {
                    let operand1 = match iter.peek() {
                        Some(Token::Label(operand)) => operand,
                        None => {
                            return Err("Expected an operand but got EOF".to_string());
                        }
                        Some(Token::Mnemonic(mnemonic)) => {
                            return Err(format!(
                                "Expected an operand but got mnemonic '{}'",
                                mnemonic.name
                            ));
                        }
                        Some(Token::Data(data)) => {
                            return Err(format!("Expected an operand but got data '{}'", data));
                        }
                    };

                    iter.next();

                    if let Some(_) = mnemonic.operands[1] {
                        let operand2 = match iter.peek() {
                            Some(Token::Label(operand)) => operand,
                            None => {
                                return Err("Expected an operand but got EOF".to_string());
                            }
                            Some(Token::Mnemonic(mnemonic)) => {
                                return Err(format!(
                                    "Expected an operand but got mnemonic '{}'",
                                    mnemonic.name
                                ));
                            }
                            Some(Token::Data(data)) => {
                                return Err(format!("Expected an operand but got data '{}'", data));
                            }
                        };

                        iter.next();

                        program.instructions.push(Instruction {
                            label: None,
                            mnemonic: *mnemonic,
                            operands: [Some(operand1.clone()), Some(operand2.clone())],
                        });
                    } else {
                        program.instructions.push(Instruction {
                            label: None,
                            mnemonic: *mnemonic,
                            operands: [Some(operand1.clone()), None],
                        });
                    }
                }
            }
            Token::Data(data) => {
                return Err(format!(
                    "Expected a label or an mnemonic but got data '{}'",
                    data
                ));
            }
        };
    }

    return Ok(program);
}

#[cfg(test)]
mod test {
    use crate::{
        parser::{self, Instruction, LabeledData, Program},
        tokenizer::{MNEMONICS, Token},
    };

    #[test]
    fn simple() {
        let tokens = vec![
            Token::Mnemonic(MNEMONICS[0]),
            Token::Label(Box::from("S1")),
            Token::Label(Box::from("S1")),
            Token::Label(Box::from("S1")),
            Token::Data(Box::from("1")),
        ];

        let result = parser::parse(tokens);
        assert!(result.is_ok());
        let result = result.unwrap();

        let program = Program {
            instructions: vec![Instruction {
                mnemonic: MNEMONICS[0],
                label: None,
                operands: [Some(Box::from("S1")), Some(Box::from("S1"))],
            }],
            data: vec![LabeledData {
                label: Box::from("S1"),
                data: 1,
            }],
        };

        assert_eq!(result, program);
    }
}
