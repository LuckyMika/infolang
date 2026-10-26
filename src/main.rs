use clap::Parser;
mod executor;
mod parser;
mod tokenizer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,
    #[arg(short, long)]
    tokenize: bool,
    #[arg(short, long)]
    parse: bool,
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
            eprintln!("Error during tokenization:");
            eprintln!("{e}");
            return;
        }
    };

    if args.tokenize {
        println!("Tokens:");

        for token in &tokens {
            println!("{:?}", token);
        }
    }

    let program = match parser::parse(tokens) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error during parsing:");
            eprintln!("{e}");
            return;
        }
    };

    if args.parse {
        println!("Program:");
        println!("{:?}", program);
    }
}
