mod grep;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    text: String,
    file_path: String,
    #[arg(short, long)]
    recursive: bool,
    #[arg(long)]
    no_color: bool,
    #[arg(long,short)]
    count: bool
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    grep::run(args)
}
