use std::fs::File;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::PathBuf;
use crate::Args;
use crate::stats::Stats;

pub fn run(args: Args) -> std::io::Result<()> {
    for file in args.file_paths.iter() {
        match run_file(&args, file) {
            Err(error) => eprint!("{}",error),
            _ => ()
        }
    }

    Ok(())
}

fn run_file(args: &Args, file_path: &String) -> std::io::Result<()> {
    let mut stats = &mut Stats::new();

    if !args.recursive {
        grep_file(&args,file_path,&mut stats)?;
    } else {
        let path = PathBuf::from(file_path);
        grep_dir(path, &args, &mut stats);
    }

    if args.count {
        println!("{}", stats.matches);
    }

    Ok(())
}

fn grep_dir(path: PathBuf, args: &Args, mut stats: &mut Stats) {
    let paths = fs::read_dir(path).unwrap();
    for p in paths {
        let p = p.unwrap().path();
        if p.is_dir() {
            grep_dir(p, args, &mut stats);
        } else {
            grep_file(&args, &p.display().to_string(), &mut stats).unwrap_or_else(|err| {
                eprintln!("Error reading file {} ({err})", p.display())
            });
        }
    }
}

fn grep_file(args: &Args, file_path: &String, stats: &mut Stats) -> std::io::Result<()> {
    let path = PathBuf::from(file_path);
    let file = File::open(&path)?;
    let mut reader = BufReader::new(&file);

    loop {
        let mut chunk = Vec::new();
        let n = reader.read_until(10,&mut chunk)?;

        if n == 0 { break; }

        let mut line = String::from_utf8_lossy(&chunk).to_string();

        if line.contains(&args.text) {
            stats.matches += 1;
            if !args.no_color {
                line = line.replace(&args.text, &args.text.red().to_string());
            }
            if !args.count {
                print!("{}",line);
            }
        }
    }
    Ok(())
}

/**
 * Alternative implementation for grep_file.
 * Only works for text files (breaks with non UTF-8 characters)
 */
fn _grep_text_file(file_path: String, text: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

     for line in reader.lines() {
         let line = line?;
         if line.contains(&text) {
             println!("{}", line);
         }
     }
    Ok(())
}
