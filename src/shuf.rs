use std::io::{Read, StdinLock};
use rand::Rng;
use rand::thread_rng;
use std::fs::File;
use rand::seq::SliceRandom;
use std::path::Path;

pub trait IntoReader {
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

/// Returns a random permutation of an Iterable [`input`].
/// The whole Iterator needs to be consumed before permuting.
///
/// # Arguments
///
/// * `input` - an Iterable.
pub fn read_from_iter<T, E, I>(input: I) -> Result<Vec<T>, E>
    where I: Iterator<Item = Result<T, E>>
{
    let mut rng = thread_rng();
    let mut items: Vec<T> = Vec::new();

    for item in input {
        match item {
            Ok(valid) => {
                items.push(valid)
            }
            Err(_e) => panic!("Invalid input")
        }
    }
    items.shuffle(&mut rng);
    Ok(items)
}

/// Returns a random permutation of [`items`] elements from an Iterable [`input`].
///
/// # Arguments
///
/// * `input` - an Iterable
/// * `items` - number of elements to sample (the reservoir size).
pub fn sample_from_iter<T, E, I>(input: I, items: usize) -> Result<Vec<T>, E>
    where
        I: Iterator<Item = Result<T, E>>
{
    let mut rng = rand::thread_rng();
    let mut reservoir: Vec<T> = Vec::with_capacity(items);
    // TODO(gmodena, 2019-12-22) - think about an efficient way to copy first n items from iterator
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
            Err(_e) => panic!("Invalid input")
        }
    }
    Ok(reservoir)
}



#[cfg(test)]
mod tests {
    use super::sample_from_iter;
    use super::read_from_iter;

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
