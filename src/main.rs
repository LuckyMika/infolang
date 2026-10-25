use clap::Parser;
mod parser;
mod tokenizer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();

    let contents = match std::fs::read_to_string(&args.file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", &args.file, e);
            return;
        }
    };

    let tokens = match tokenizer::tokenize(contents) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error during tokanization:");
            eprintln!("{e}");
            return;
        }
    };

    println!("Tokens:");

    for token in &tokens {
        println!("{:?}", token);
    }

    let program = match parser::parse(tokens) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error during parsing:");
            eprintln!("{e}");
            return;
        }
    };

    println!("Program:");
    println!("{:?}", program);
}
