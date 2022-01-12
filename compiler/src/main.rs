use std::{time::Instant, io::Write};

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Args {
    #[structopt(short="i", long="input", parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(short="o", long="output", default_value="out.bin", parse(from_os_str))]
    output: std::path::PathBuf,
}


fn main() {
    let args = Args::from_args();
    let input = std::fs::read_to_string(args.input).expect("Unable to read input file");
    let mut output = std::fs::File::create(args.output).expect("Unable to create output file");

    let timer = Instant::now(); 

    let tokens = compiler::lexer::Lexer { input: input.as_str() }.lex();

    let mut parser = compiler::Parser { input: tokens };

    let result = parser.parse();

    if let Err(err) = result {
        let prefix = format!("{}: on line {}: ", "ERROR", err.responsible.line + 1);

        println!("{}{}", prefix.red().bold(), input.lines().nth(err.responsible.line).unwrap());
        println!("{}{} {}", " ".repeat(prefix.len() + err.responsible.range.start), "^".repeat(err.responsible.range.len()).red().bold(), err.cause.red().bold());
    } else if let Ok(recipe) = result {
        let mut out = Vec::new();
        compiler::to_bytes(recipe, &mut out);

        output.write_all(out.as_slice()).unwrap();
        println!("{}", format!("Compilation successful! TIME: {} seconds", Instant::now().duration_since(timer).as_secs_f32()).bright_green().bold());
    }
}
