mod rainbo_wc;

use clap::Parser;
use rainbo_wc::{
    file_reader::get_data, file_statistics::FileStatistics, rainbow_writer::RainbowWriter,
};
use std::process;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    words: bool,

    #[arg(short, long)]
    lines: bool,

    #[arg(short = 'c', long)]
    bytes: bool,

    #[arg(short = 'm', long)]
    chars: bool,

    #[arg(
        required = false,
        default_value = "",
        help = "File to read, omit if content is provided via standard input"
    )]
    file: String,
}

fn main() {
    let mut args = Args::parse();
    let mut text_writer = RainbowWriter::new();

    if !args.words && !args.lines && !args.bytes && !args.chars {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let file = match get_data(&args.file) {
        Ok(data) => data,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                if args.file.is_empty() {
                    println!("No file provided, use -h to see options");
                    process::exit(1);
                } else {
                    println!("File not found: {}", args.file);
                    process::exit(1);
                }
            }
            _ => {
                println!("Error reading file: {}", e);
                process::exit(1);
            }
        },
    };

    let statistics = FileStatistics::get_string_statistics(file).unwrap();

    if args.lines {
        text_writer.write(format!("{} ", statistics.lines).as_str());
    }

    if args.words {
        text_writer.write(format!("{} ", statistics.words).as_str());
    }

    if args.chars {
        text_writer.write(format!("{} ", statistics.chars).as_str());
    }

    if args.bytes {
        text_writer.write(format!("{} ", statistics.bytes).as_str());
    }

    if !args.file.is_empty() {
        text_writer.write(format!("{} ", args.file).as_str());
    }

    text_writer.writeln("");
}
