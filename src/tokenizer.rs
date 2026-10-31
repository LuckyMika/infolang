#[derive(Debug, PartialEq)]
pub enum Token {
    Label(Box<str>),
    Mnemonic(Mnemonic),
    Data(Box<str>),
}

#[derive(Debug, PartialEq)]
pub enum Token2 {
    OperatorLabel(Box<str>),
    LocationLabel(Box<str>),
    Mnemonic(Box<str>),
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

pub fn tokenize_v2<'a>(contents: &'a str) -> Result<Vec<Token2>, String> {
    let mut tokens = Vec::with_capacity(20);

    for (line_num, line) in contents.lines().enumerate() {
        let mut split = line.split(":").peekable();
        let first = split.next();
        let second = split.next();

        if split.next().is_some() {
            return Err(format!(
                "Error in line {line_num}: line can at most have 1 label definition"
            ));
        }

        let Some(first) = first else {
            return Err(format!("Error in line {line_num}: Empty line in iteration"));
        };

        let (label, instruction) = if let Some(instruction) = second {
            (Some(first), instruction)
        } else {
            (None, first)
        };

        dbg!(instruction.trim_ascii());

        if instruction
            .trim_ascii()
            .chars()
            .next()
            .is_some_and(|c| c >= '0' && c <= '9' || c == '-')
        {
            let Some(label) = label else {
                return Err(format!(
                    "Error in line {line_num}: Lone data without a label"
                ));
            };

            tokens.push(Token2::LocationLabel(Box::from(label)));
            tokens.push(Token2::Data(Box::from(instruction.trim_ascii())));
            continue;
        }

        if let Some(label) = label {
            tokens.push(Token2::LocationLabel(Box::from(label)));
        }

        let mut buffer = String::new();
        let mut mnemonic_pushed = false;

        for character in instruction.chars() {
            if character == ' ' || character == ',' {
                if buffer.len() == 0 {
                    continue;
                }

                if !mnemonic_pushed {
                    tokens.push(Token2::Mnemonic(
                        std::mem::take(&mut buffer).into_boxed_str(),
                    ));

                    mnemonic_pushed = true;
                    continue;
                } else {
                    tokens.push(Token2::OperatorLabel(
                        std::mem::take(&mut buffer).into_boxed_str(),
                    ));
                    continue;
                }
            }

            buffer.push(character);
        }

        if buffer.len() != 0 {
            if !mnemonic_pushed {
                tokens.push(Token2::Mnemonic(
                    std::mem::take(&mut buffer).into_boxed_str(),
                ));
            } else {
                tokens.push(Token2::OperatorLabel(
                    std::mem::take(&mut buffer).into_boxed_str(),
                ));
            }
        }
    }

    Ok(tokens)
}

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

#[cfg(test)]
mod test_v2 {

    mod simple {
        use crate::tokenizer::{self, Token2};

        #[test]
        fn add() {
            let input = "ADD S1, S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn mnemonic_extra_spacing() {
            let input = "ADD        S1, S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operands_extra_spacing() {
            let input = "ADD S1     , S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operand_extra_spacing() {
            let input = "ADD S1,          S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn trailing_spaces() {
            let input = "ADD S1, S1              ";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operand_trailing_commas() {
            let input = "ADD S1, S1,";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }

        #[test]
        fn operands_trailing_commas() {
            let input = "ADD S1,, S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }
    }

    mod label {
        use crate::tokenizer::{self, Token2};

        #[test]
        fn add() {
            let input = "P1: ADD S1, S1";

            let result = tokenizer::tokenize_v2(input);

            assert!(result.is_ok());

            let result = result.unwrap();
            let expected = vec![
                Token2::LocationLabel(Box::from("P1")),
                Token2::Mnemonic(Box::from("ADD")),
                Token2::OperatorLabel(Box::from("S1")),
                Token2::OperatorLabel(Box::from("S1")),
            ];

            assert_eq!(result, expected);
        }
    }
}
