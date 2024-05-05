mod grep;
mod stats;

use std::process::exit;

use clap::Parser;

fn validate_files(x: &str) -> Result<String, String> {
    println!("Validating files: {}", x);
    // if x.contains(" ") {
    //     return Err("File paths cannot contain spaces".to_string())
    // }
    // Ok(x.to_string())
    Ok(String::from("balblabla"))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    text: String,
    #[clap(value_parser = validate_files)]
    file_paths: Vec<String>,
    #[arg(short, long,)]
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
