use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.file.len();
    for (file_num, filename) in config.file.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{filename} {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                    // 以下のようにすると、改行コードがLFに強制されてしまう
                    // for line in file.lines().take(config.lines) {
                    //     let line = line?;
                    //     println!("{line}");
                    // }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    file: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file(s)
    #[arg(num_args = 0.., default_value = "-", value_name = "files")]
    file: Vec<String>,

    /// Number of lines
    #[arg(short = 'n', long, default_value = "10")]
    lines: usize,

    /// Number of bytes
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    if cli.lines == 0 {
        return Err("Number of lines cannot be 0".into());
    }

    if let Some(0) = cli.bytes {
        return Err("Number of bytes cannot be 0".into());
    }

    Ok(Config {
        file: cli.file,
        lines: cli.lines,
        bytes: cli.bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

// derive を使う場合は型変換が自動で行われるので書籍にある以下のコードは不要
// fn parse_positive_int(str: &str) -> MyResult<usize> {
//     match str.parse() {
//         Ok(int) if int > 0 => Ok(int),
//         _ => Err(From::from(str)),
//     }
// }
//
// #[test]
// fn test_parse_positive_int() {
//     let res = parse_positive_int("3");
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), 3);
//
//     let res = parse_positive_int("foo");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
//
//     let res = parse_positive_int("0");
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }