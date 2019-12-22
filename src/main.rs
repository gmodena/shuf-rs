extern crate rand;
extern crate log;

// TODO
// - Refactor to Traits for polymorfism (IO, sampling vs. reading) IN PROGRESS
// - Pattern matching for Result and Option types. DONE

// - Get input file from stdin. DONE
// - Parse arguments: DONE
// - Implement reads from stdin. DONE
// - Read whole file / IN: DONE
// - Shuffle whole file TODO
// - Format output DONE
// - Unit tests: TODO

use std::io::{self, Read, StdinLock};
use rand::Rng;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;
use rand::seq::SliceRandom;
use std::path::Path;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Number of lines to read
    #[structopt(short = "n", long = "--head-count=COUNT")]
    num: Option<usize>,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

/// Returns a random permutation of an Iterable [`input`].
/// The whole Iterator needs to be consumed before permuting.
fn read_from_iter<T, E, I>(input: I) -> Result<Vec<T>, E>
    where I: Iterator<Item = Result<T, E>>
{
    let mut rng = thread_rng();
    let mut items: Vec<T> = Vec::new();

    for item in input {
        match item {
            Ok(valid) => {
                items.push(valid)
            }
            Err(e) => panic!("Invalid input")
        }
    }
    items.shuffle(&mut rng);
    Ok(items)
}

/// Returns a random permutation of [`items`] elements from an Iterable [`input`].
fn sample_from_iter<T, E, I>(input: I, items: usize) -> Result<Vec<T>, E>
where
    I: Iterator<Item = Result<T, E>>
{
    let mut rng = rand::thread_rng();
    let mut reservoir: Vec<T> = Vec::with_capacity(items);
    // TODO(gmodena, 2019-10-26) - think about an efficient way to copy first n items from iterator
    //                             into the reservoir .
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
            Err(e) => panic!("Invalid input")
        }
    }
    Ok(reservoir)
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

impl<'a> IntoReader for StdinLock<'a> {
    type OutReader = StdinLock<'a>;

    fn into_reader(self) -> StdinLock<'a> {
        self
    }
}

struct Shuffler {
    num: Option<usize>,
}

impl Shuffler {
    pub fn new() -> Shuffler {
        Shuffler {
            num: None,
        }
    }

    pub fn with_num<'a>(&'a mut self, arg: usize) -> &'a mut Shuffler {
        self.num = Some(arg);
        self
    }

    pub fn shuffle<I>(&mut self, data: I) -> Result<Vec<String>, std::io::Error>
        where I: IntoReader, I:: OutReader: Read {
        let local = BufReader::new(data.into_reader());

        match self.num {
            Some(n) => sample_from_iter(local.lines(), n),
            None => read_from_iter(local.lines())
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


#[cfg(test)]
mod tests {
    use super::sample_from_iter;
    use super::read_from_iter;
    use std::io::{BufReader, BufRead};

    #[test]
    fn test_sample_from_iter() {
        let n = 10;
        let  items: Vec<Result<usize, std::io::Error>> = (0..100).map(
            |_| { Ok(rand::random::<usize>()) }).collect();

        let shuffled = sample_from_iter(items.into_iter(), n);

        assert_eq!(shuffled.unwrap().len(), n);
    }

    #[test]
    fn test_read_from_iter() {
        let n = 100;
        let  items: Vec<Result<usize, std::io::Error>> = (0..n).map(
            |_| { Ok(rand::random::<usize>()) }).collect();

        let shuffled = read_from_iter(items.into_iter());

        assert_eq!(shuffled.unwrap().len(), n);
    }
}