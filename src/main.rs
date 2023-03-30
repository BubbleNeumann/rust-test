/*
* Implement basic function to split some generic computational work between threads.
* Split should occur only on some threshold - if computational work (input length)
* is shorter than this threshold, no splitting should occur and no threads should be created.
*
* You get as input:
*
* 1. Vec<T>
* 2. Function f(t: T) -> R
*
* Threshold can be just constant.
*
* You should return:
*   1. Up to you, but probably some Vec of the same length as input(1)
*
* Code should be published on github.
*/

use std::cmp::min;
use std::io::stdin;
use std::thread::{self, JoinHandle};

fn f(t: &i32) -> i32 {
    t.pow(2)
}

fn spawn_upon_threshold(vec: &Vec<i32>, f: fn(&i32) -> i32) -> Vec<i32> {
    // consider threshold to be a computational limitation of 1 thread
    const THRESHOLD: usize = 5;

    let f_caller = move |vec: Vec<i32>| vec.iter().map(|x| f(x)).collect::<Vec<i32>>();

    // if size is above threshold, spawn threads first, so we don't waste time
    let mut handles: Vec<JoinHandle<Vec<i32>>> = vec![];
    for i in 1..vec.len() / THRESHOLD + (vec.len() % THRESHOLD > 0) as usize {
        let chunk = vec[i * THRESHOLD..min((i + 1) * THRESHOLD, vec.len())].to_vec();
        let handle = thread::spawn(move || f_caller(chunk));
        handles.push(handle);
    }

    // handle the first chunk of the input vector
    let mut res: Vec<i32> = f_caller(vec[0..min(THRESHOLD, vec.len())].to_vec());

    // merge threads if any were spawn and concat their return values into res
    for handle in handles {
        res.extend(handle.join().unwrap());
    }

    res
}

fn main() {
    // read Vec from console
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect("");
    let vec: Vec<_> = inp
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", spawn_upon_threshold(&vec, f));
}

#[cfg(test)]
mod tests {
    use crate::{f, spawn_upon_threshold};

    #[test]
    fn test_spawn_upon_threshold_empty_vec() {
        let inp = vec![];
        let expected = vec![];
        assert_eq!(spawn_upon_threshold(&inp, f), expected);
    }

    #[test]
    fn test_spawn_upon_threshold_below_threshold() {
        let inp = vec![1, 2, 3];
        let expected = vec![1, 4, 9];
        assert_eq!(spawn_upon_threshold(&inp, f), expected);
    }

    #[test]
    fn test_spawn_upon_threshold_eq_to_threshold() {
        let inp = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 4, 9, 16, 25];
        assert_eq!(spawn_upon_threshold(&inp, f), expected);
    }

    #[test]
    fn test_spawn_upon_threshold_eq_to_double_threshold() {
        let inp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected = vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
        assert_eq!(spawn_upon_threshold(&inp, f), expected);
    }

    #[test]
    fn test_spawn_upon_threshold_above_double_threshold() {
        let inp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let expected = vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121];
        assert_eq!(spawn_upon_threshold(&inp, f), expected);
    }
}
