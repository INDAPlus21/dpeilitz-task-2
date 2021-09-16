/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

// Solution inspired by Elias Floreteng's

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input as a vector
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut numbers: Vec<i32> = input
        .lock()
        .lines()
        .skip(1) // read only number line
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<i32>().ok().unwrap())
        .collect();

    //sort numbers
    numbers.sort();
    // sum all numbers larger than numbers.len()/2 by iterating through the vector
    let sum: i32 = numbers[(numbers.len()) / 2..].iter().sum();
    //print sum
    println!("{}", sum)
}
