extern crate rand;

use std::io;
use structopt::StructOpt;
use std::path::Path;

mod shuf;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Number of lines to read
    #[structopt(short = "n", long = "--head-count=COUNT")]
    num: Option<usize>,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let mut s = shuf::Shuffler::new();

    if args.num.is_some() {
        s.with_num(args.num.unwrap());
    }

    let sample = match args.path {
        Some (path) =>  s.shuffle(Path::new(&path)),
        None => s.shuffle(io::stdin().lock())
    };


    for s in sample.expect("Invalid record") {
        println!("{}", s);
    }
    Ok(())
}


