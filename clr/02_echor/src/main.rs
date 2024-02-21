use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input text
    #[arg(required = true, num_args=1..)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short)]
    n: bool,
}

fn main() {
    let cli = Cli::parse();
    print!("{}", cli.text.join(" "));
    if !cli.n {
        println!();
    }

    // println!("{:#?}", cli);

    // 本文のコード：
    // let cli = Cli::parse();
    // let text = cli.text;
    // let omit_newline = cli.n;
    // print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}


// fn main() {
//     println!("{:?}", std::env::args());
// }