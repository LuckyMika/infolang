#[derive(Debug)]
pub enum Token {
    Label(Box<str>),
    Mnemonic(Mnemonic),
    Data(Box<str>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mnemonic {
    pub name: &'static str,
    pub operands: [Option<Operand>; 2],
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operand {
    CodeLabel,
    DataLabel,
}

pub static MNEMONICS: [Mnemonic; 9] = [
    Mnemonic {
        name: "ADD",
        operands: [Some(Operand::DataLabel), Some(Operand::DataLabel)],
    },
    Mnemonic {
        name: "SUB",
        operands: [Some(Operand::DataLabel), Some(Operand::DataLabel)],
    },
    Mnemonic {
        name: "SPRUNG",
        operands: [Some(Operand::CodeLabel), None],
    },
    Mnemonic {
        name: "SPRUNG0",
        operands: [Some(Operand::CodeLabel), Some(Operand::DataLabel)],
    },
    Mnemonic {
        name: "SPRUNGNEG",
        operands: [Some(Operand::CodeLabel), Some(Operand::DataLabel)],
    },
    Mnemonic {
        name: "INIT",
        operands: [Some(Operand::DataLabel), None],
    },
    Mnemonic {
        name: "RUECKGABE",
        operands: [Some(Operand::DataLabel), None],
    },
    Mnemonic {
        name: "INKREMENT",
        operands: [Some(Operand::DataLabel), None],
    },
    Mnemonic {
        name: "DEKREMENT",
        operands: [Some(Operand::DataLabel), None],
    },
];

#[derive(PartialEq, Eq)]
enum TokenizerState {
    LabelOrMnemonic,
    MnemonicOrData,
    Mnemonic,
    Data,
    Operand,
    Operands,
    EOL,
}

// [Label:] Instruction [Operand][,Operands]?

pub fn tokenize(contents: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::with_capacity(20);
    let mut state = TokenizerState::LabelOrMnemonic;

    for line in contents.lines() {
        let mut string = String::new();

        for c in line.chars() {
            match state {
                TokenizerState::LabelOrMnemonic => {
                    if c == ':' {
                        if string.len() > 0 {
                            tokens.push(Token::Label(Box::from(string.as_str())));

                            state = TokenizerState::MnemonicOrData;
                            string = String::new();
                            continue;
                        } else {
                            return Err("Label name cannot be empty!".to_string());
                        }
                    } else if c == ' ' {
                        if string.len() == 0 {
                            continue;
                        }
                        if let Some(mnemonic) = MNEMONICS
                            .iter()
                            .find(|mnemonic| mnemonic.name == string.to_uppercase().as_str())
                        {
                            tokens.push(Token::Mnemonic(*mnemonic));

                            if let Some(_) = mnemonic.operands[0].as_ref() {
                                if let Some(_) = mnemonic.operands[1].as_ref() {
                                    state = TokenizerState::Operands;
                                    string = String::new();
                                    continue;
                                } else {
                                    state = TokenizerState::Operand;
                                    string = String::new();
                                    continue;
                                }
                            } else {
                                state = TokenizerState::EOL;
                                string = String::new();
                                continue;
                            }
                        } else {
                            return Err(format!("'{}' is not a valid mnemonic!", &string));
                        }
                    }

                    string.push(c);
                }
                TokenizerState::MnemonicOrData => {
                    if c == ' ' {
                        continue;
                    }

                    if c >= '0' && c <= '9' || c == '-' {
                        state = TokenizerState::Data;
                        string.push(c);
                    } else {
                        state = TokenizerState::Mnemonic;
                        string.push(c);
                    }
                }
                TokenizerState::Data => {
                    if c >= '0' && c <= '9' || c == '-' {
                        string.push(c);
                        if c == line.chars().last().expect("Empty line in iteration") {
                            tokens.push(Token::Data(Box::from(string.as_str())));
                        }
                    } else {
                        return Err(format!("{c} is not valid data!"));
                    }
                }
                TokenizerState::Mnemonic => {
                    if c == ' ' {
                        if string.len() == 0 {
                            continue;
                        }
                        if let Some(mnemonic) = MNEMONICS
                            .iter()
                            .find(|mnemonic| mnemonic.name == string.to_uppercase().as_str())
                        {
                            tokens.push(Token::Mnemonic(*mnemonic));

                            if let Some(_) = mnemonic.operands[0].as_ref() {
                                if let Some(_) = mnemonic.operands[1].as_ref() {
                                    state = TokenizerState::Operands;
                                    string = String::new();
                                    continue;
                                } else {
                                    state = TokenizerState::Operand;
                                    string = String::new();
                                    continue;
                                }
                            } else {
                                state = TokenizerState::EOL;
                                string = String::new();
                                continue;
                            }
                        } else {
                            return Err(format!("'{}' is not a valid mnemonic!", &string));
                        }
                    }

                    string.push(c);
                }
                TokenizerState::Operand => {
                    if c == ' ' && string.len() == 0 {
                        continue;
                    }
                    if c == line.chars().last().expect("Empty line in iteration") {
                        string.push(c);

                        tokens.push(Token::Label(Box::from(string.as_str())));
                        state = TokenizerState::EOL;
                        string = String::new();
                        continue;
                    }

                    string.push(c);
                }
                TokenizerState::Operands => {
                    if c == ' ' && string.len() == 0 {
                        continue;
                    }
                    if c == ',' {
                        tokens.push(Token::Label(Box::from(string.as_str())));
                        state = TokenizerState::Operand;
                        string = String::new();
                        continue;
                    }

                    string.push(c);
                }
                TokenizerState::EOL => {
                    if c != ' ' && c != ',' {
                        return Err(format!("Expected EOL but got {c}"));
                    }
                }
            };
        }

        state = TokenizerState::LabelOrMnemonic;
    }

    Ok(tokens)
}
