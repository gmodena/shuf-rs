#[macro_use] extern crate lazy_static;

extern crate rand;
extern crate log;
extern crate regex;

// TODO
// - Refactor to Traits for polymorfism (IO, sampling vs. reading) IN PROGRESS
// - Pattern matching for Result and Option types. DONE
// - Copy first n items from iterator. TODO: find an efficient way
// - Get input file from stdin. DONE
// - Parse arguments: DONE
// - Implement reads from stdin. DONE
// - Read whole file / IN: DONE
// - Shuffle whole file TODO
// - Format output DONE
// - Unit tests: TODO

use std::io::{self, Stdin, StdinLock};
use rand::Rng;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use log::{warn, error};
use structopt::StructOpt;
use rand::seq::SliceRandom;
use std::str::FromStr;
use regex::Regex;
use std::string::ParseError;
use std::path::Path;
use std::io::Read;
use std::fmt::Error;
use std::option::Iter;
use std::ops::Range;
use std::slice;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Number of lines to read
    #[structopt(short = "n", long = "--head-count=COUNT")]
    num: Option<usize>,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

#[derive(Debug)]
struct InputRange {
    low: u32,
    high: u32
}

impl FromStr for InputRange {
    type Err = ParseError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        };
        let parse_error_msg= "Could not parse input range";

        let captures = RE.captures(input).unwrap();

        let low = captures.get(1).map_or("", |m| m.as_str())
            .parse::<u32>()
            .expect(parse_error_msg);
        let high = captures.get(2).map_or("", |m| m.as_str())
            .parse::<u32>()
            .expect(parse_error_msg);

        Ok(InputRange { low,  high })
    }
}

fn print_shuffled(items: Vec<String>) {
    for s in items {
        println!("{}", s);
    }
}

/// Generate a random permutation of an Iterable.
/// Currently the whole Iterator needs to be consumed before permuting.
fn read_from_iter<I>(input: I) -> Vec<String>
    where I: Iterator<Item = Result<String>> {
    let mut rng = thread_rng();
    let mut items: Vec<String> = Vec::new();

    for item in input {
        match item {
            Ok(valid) => {
                items.push(valid)
            }
            Err(e) => warn!("{}", e)
        }
    }
    items.shuffle(&mut rng);
    return items;
}

fn sample_from_iter<I>(input: I, items: usize) -> Vec<String>
where I: Iterator<Item = Result<String>>
{
    let mut rng = rand::thread_rng();
    let mut reservoir: Vec<String> = Vec::with_capacity(items);

    for (i, item) in input.enumerate() {
        match item  {
            Ok(valid) => {
                let j = rng.gen_range(0, i+1);
                if i < items {
                    reservoir.push(valid);
                } else if j < items {
                    reservoir[j] = valid;
                }
            }
            Err(e) => warn!("{}", e)
        }
    }
    return reservoir;
}

trait IntoReader {
    type OutReader: Read;

    fn into_reader(self) -> Self::OutReader;
}

impl<'a> IntoReader for  &'a Path {
    type OutReader = File;

    fn into_reader(self) -> File {
        File::open(self).unwrap()
    }
}

// TODO(gmodena): we should use StdinLock instead
impl IntoReader for Stdin {
    type OutReader = Stdin;

    fn into_reader(self) -> Stdin {
        io::stdin()
    }
}

struct Shuffler {
    num: Option<usize>,
    input_range: Option<InputRange>
}

impl Shuffler {
    pub fn new() -> Shuffler {
        Shuffler {
            num: None,
            input_range: None
        }
    }

    pub fn with_num<'a>(&'a mut self, arg: usize) -> &'a mut Shuffler {
        self.num = Some(arg);
        self
    }

    pub fn with_input_range<'a>(&'a mut self, arg: InputRange) -> &'a mut Shuffler {
        self.input_range = Some(arg);
        self
    }

    pub fn shuffle<I>(&mut self, data: I) -> Result<Vec<String>>
        where I: IntoReader, I:: OutReader: Read {
        if self.input_range.is_some() && self.num.is_some() {
            error!("Conflicting arguments. '-n' and '-i' are mutually exclusive ");
            Error;
        }

        let local = BufReader::new(data.into_reader());

        match self.num {
            Some(n) => Ok(sample_from_iter(local.lines(), n)),
            None => Ok(read_from_iter(local.lines()))
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let sample;

    let mut s = Shuffler::new();

    match args.num {
        Some(n) => s.with_num(n),
        None => s.with_num(10)
    };

    match args.path {
        Some (path) => sample = s.shuffle(Path::new(&path)),
        None => sample = s.shuffle(io::stdin())
    }

    //println!("{:?}, {:?}", sample, sample.ok().unwrap().len());
    print_shuffled(sample.ok().expect("Aa"));

    Ok(())
}


#[cfg(test)]
mod tests {
}