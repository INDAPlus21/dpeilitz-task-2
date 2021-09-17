/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

// HEAVILY INSPIRED BY FELIX MURNION'S SOLUTION

use std::io;
use std::io::prelude::*;
use std::char;

// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let numbers: Vec<usize> = input
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<usize>().ok().unwrap())
        .collect();

    let rows: usize = numbers[0];
    let columns: usize = numbers[1];

    let mut output = String::with_capacity((rows) * (columns + 1));

    for x in 1..=rows {
        for y in 1..=columns {
            //skriv avstånd upp ned sida och sida och genom let ...
            let dist_top = x;
            let dist_left = y;
            let dist_bot = (rows+1)  - dist_top; //add one to measure the distance OUTSIDE of the box
            let dist_right = (columns+1) - dist_left;
            //find out the shortest distance
            let min = dist_left.min(dist_right).min(dist_top).min(dist_bot);
            if min < 10 {
                
                //convert to char by getting the asci value, converting to u8 then to char
                output.push(char::from_digit(min as u32, 10).unwrap());
            }
            else{
                output.push('.');
            }
        }
        output.push_str("\n")
    }

    println!("{}", output);
}
