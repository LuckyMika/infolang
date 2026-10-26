use clap::Parser;
mod executor;
mod parser;
mod tokenizer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File with assembly to run
    file: String,

    /// Print the output of the tokenizer
    #[arg(short, long)]
    tokenize: bool,

    /// Print the output of the parser
    #[arg(short, long)]
    parse: bool,

    /// Suppress output of the executor
    #[arg(short, long)]
    quiet: bool,

    /// Do not execute the program
    #[arg(short = 'n', long)]
    halt: bool,
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

    if args.halt {
        return;
    }

    let (program, ret) = match executor::execute(program) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error during execution:");
            eprintln!("{e}");
            return;
        }
    };

    if !args.quiet {
        println!("Program Result:");
        if let Some(value) = ret {
            println!("Returned: {value}");
        }

        for data in program.data {
            println!("{}: {}", data.label, data.data);
        }
    }
}
