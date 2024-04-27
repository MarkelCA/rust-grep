use std::fs::File;
use colored::Colorize;
use std::io::{BufRead, BufReader};
use std::fs;
use std::path::PathBuf;
use crate::Args;

pub fn run(args: Args) -> std::io::Result<()> {

    if !args.recursive {
        grep_file(&args, &args.text)?;
    } else {
        let path = PathBuf::from(&args.file_path);
        grep_dir(path, &args);
    }

    Ok(())
}

fn grep_dir(path: PathBuf, args: &Args) {
    let paths = fs::read_dir(path).unwrap();
    for p in paths {
        let p = p.unwrap().path();
        if p.is_dir() {
            grep_dir(p, args);
        } else {
            let ar = crate::Args {
                file_path: p.display().to_string(),
                text: args.text.clone(),
                ..*args
            };

            grep_file(&ar, &args.text).unwrap_or_else(|err| {
                eprintln!("Error reading file {} ({err})", p.display())
            });
        }
    }
}


fn grep_file(args: &Args, text: &str) -> std::io::Result<()> {
    let path = PathBuf::from(&args.file_path);
    let file = File::open(&path)?;
    let mut reader = BufReader::new(&file);

    loop {
        let mut chunk = Vec::new();
        let n = reader.read_until(10,&mut chunk)?;

        if n == 0 { break; }

        let mut line =  String::from_utf8_lossy(&chunk).to_string();

        if line.contains(text) {
            if !args.no_color {
                line = line.replace(text, &text.red().to_string());
            }
            print!("{}",line);
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
