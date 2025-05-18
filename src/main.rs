mod rainbo_wc;

use clap::Parser;
use rainbo_wc::{
    file_reader::get_data, file_statistics::FileStatistics, rainbow_writer::RainbowWriter,
};

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

    #[arg()]
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

    let file = get_data(&args.file).unwrap();
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
