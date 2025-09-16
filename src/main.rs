mod compiler;
use std::path::Path;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(help = "Input brainfuck file")]
    input: String,

    #[arg(short, long, help = "Output assembly file")]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let output_file = cli.output.unwrap_or_else(|| {
        let mut name = cli.input.clone();
        if let Some(pos) = name.rfind('.') {
            name.replace_range(pos.., ".asm");
        } else {
            name.push_str(".asm");
        }
        return name;
    });

    let input_path = Path::new(&cli.input);
    let output_path = Path::new(&output_file);

    compiler::compile(input_path, output_path)
}
