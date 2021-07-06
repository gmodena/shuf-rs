extern crate rand;

use std::io;
use structopt::StructOpt;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::Read;


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

pub struct Shuffler {
    num: Option<usize>,
}

impl Shuffler {
    /// Returns a Shuffler structure that can either generate random permutations
    /// a whole iterable, or select `num` items with reservoir sampling.
    pub fn new() -> Shuffler {
        Shuffler {
            num: None,
        }
    }

    /// Builder method used to set the `num` attribute.
    pub fn with_num<'a>(&'a mut self, arg: usize) -> &'a mut Shuffler {
        self.num = Some(arg);
        self
    }

    /// Generate random permutations of `data`. Select `num` items with reservoir sampling, or 
    /// permute the whole iterable if `num` is None.
    ///
    /// # Arguments
    ///
    /// * `data` - an iterable
    pub fn shuffle<I>(&mut self, data: I) -> Result<Vec<String>, std::io::Error>
        where I: shuf::IntoReader, I:: OutReader: Read {
        let local = BufReader::new(data.into_reader());

        match self.num {
            Some(n) => shuf::sample_from_iter(local.lines(), n),
            None => shuf::read_from_iter(local.lines())
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let mut s = Shuffler::new();

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


