#[derive(Debug, PartialEq)]
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

// TODO: Rewrite into enum for easier parsing and smaller parsed program sizes (less strings)
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

// [Label:] Instruction [Operand][,Operand]
// TODO: Implement tokenizer using peekable iterator

pub fn tokenize(contents: String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::with_capacity(20);
    let mut state = TokenizerState::LabelOrMnemonic;

    for line in contents.lines() {
        let mut string = String::new();

        for (i, c) in line.chars().enumerate() {
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

                        if i == line.chars().count() - 1 {
                            tokens.push(Token::Data(Box::from(string.as_str())));
                        }
                    } else {
                        state = TokenizerState::Mnemonic;
                        string.push(c);
                    }
                }
                TokenizerState::Data => {
                    if c >= '0' && c <= '9' || c == '-' {
                        string.push(c);
                        if i == line.chars().count() - 1 {
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
                    if i == line.chars().count() - 1 {
                        if c != ' ' && c != ',' {
                            string.push(c);
                        }

                        tokens.push(Token::Label(Box::from(string.as_str())));
                        state = TokenizerState::EOL;
                        string = String::new();
                        continue;
                    }

                    if c == ' ' || c == ',' {
                        continue;
                    }

                    string.push(c);
                }
                TokenizerState::Operands => {
                    if c == ' ' {
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
                        return Err(format!("Expected EOL but got '{c}'"));
                    }
                }
            };
        }

        state = TokenizerState::LabelOrMnemonic;
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {

    mod simple {
        use crate::tokenizer::{self, MNEMONICS, Token};

        #[test]
        fn add() {
            let input = "ADD S1, S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn mnemonic_extra_spacing() {
            let input = "ADD        S1, S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operands_extra_spacing() {
            let input = "ADD S1     , S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operand_extra_spacing() {
            let input = "ADD S1,          S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn trailing_spaces() {
            let input = "ADD S1, S1              ".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operand_trailing_commas() {
            let input = "ADD S1, S1,".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operands_trailing_commas() {
            let input = "ADD S1,, S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }
    }

    mod label {
        use crate::tokenizer::{self, MNEMONICS, Token};

        #[test]
        fn add() {
            let input = "P1: ADD S1, S1".to_string();

            let result = tokenizer::tokenize(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token::Label(Box::from("P1")),
                Token::Mnemonic(MNEMONICS[0]),
                Token::Label(Box::from("S1")),
                Token::Label(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }
    }
}
