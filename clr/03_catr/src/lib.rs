use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut last_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => {
                eprintln!("Failed to open {filename}: {err}")
            }
            Ok(file) => {
                for line in file.lines() {
                    let line = line?;
                    if config.number_lines {
                        last_num += 1;
                        println!("{:6}\t{line}", last_num);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:6}\t{line}", last_num);
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file(s)
    #[arg(num_args = 1.., default_value = "-")]
    file: Vec<String>,

    /// Number lines
    #[arg(short, long)]
    number: bool,

    /// Number non-blank lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();
    Ok(Config {
        files: cli.file,
        number_lines: cli.number,
        number_nonblank_lines: cli.number_nonblank,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}