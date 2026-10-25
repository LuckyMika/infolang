use crate::tokenizer::{Mnemonic, Operand, Token};

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    data: Vec<LabeledData>,
}

#[derive(Debug)]
pub struct Instruction {
    label: Option<Box<str>>,
    mnemonic: Mnemonic,
    operands: [Option<Box<str>>; 2],
}

#[derive(Debug)]
pub struct LabeledData {
    label: Box<str>,
    data: i64,
}

pub enum ParserState {
    Beginning,
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
                    println!("Parsing data {data}");
                    program.data.push(LabeledData {
                        label: label.clone(),
                        data: data.parse().expect("invalid data"),
                    });

                    iter.next();
                }
                Some(Token::Mnemonic(mnemonic)) => {
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
