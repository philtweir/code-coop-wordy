/**
 * wordy mini
 *
 * A version of wordy that only handles strings of the form
 *   What is NUM1 OP NUM2?
 * where OP is a one-word operator.
 *
 */

use std::collections::HashMap;
use std::vec::Vec;
use std::io;
use std::io::prelude::*;


fn main() {
    /* I'd probably just to do a switch approach since we are looking at a
     * fairly fixed set of options, but since there was quite
     * a bit of interest in closures and hashmaps, that's kinda pretty :) */
    let mut operations: HashMap<&str, Box<Fn(f64, f64) -> f64>> = HashMap::new();
    operations.insert("plus", Box::new(|x: f64, y: f64| x + y));
    operations.insert("minus", Box::new(|x: f64, y: f64| x - y));
    operations.insert("times", Box::new(|x: f64, y: f64| x * y));
    operations.insert("over", Box::new(|x: f64, y: f64| x / y));
    /* This is a bit like dealing with a std::map in C++.
     * Two things to notice though - firstly the operations hashmap declaration
     * and secondly the closures we use to append to the HashMap.
     *
     * The HashMap is a 'Generic' (like a template class in C++) that can
     * contain things of various types. This one maps &str (a fundamental
     * string type in Rust) to Box<Fn(f64, f64) -> f64>. That second one
     * is our closure.
     *
     * Strictly, Rust's closures (also like lambdas elsewhere) could be as simple as
     * |x, y| x + y
     * but we want to add a few of these into the same HashMap, so we need to make
     * sure the types are explicitly defined. We have functions that take
     * two floats to a float... the type of this closure is (Fn(f64, f64) -> f64)
     * The Box wrapper is about allocating on the heap instead of the stack - this
     * is an important technical distinction, but you can read more here:
     * https://doc.rust-lang.org/book/the-stack-and-the-heap.html
     */

    let stdin = io::stdin();

    /* Keep reading lines from the terminal and try and process each */
    for s in stdin.lock().lines() {
        /* unwrap is important as lines() returns something we can use to
         * get an error message or success status. If successful, we unwrap
         * the delicious string inside from our Yay It Worked object. */
        let input = s.unwrap();

        /* We take this opportunity to normalize, making this a bit more flexible.
         * Obviously, this is a basic approach to dealing with punctuation, but
         * in the spirit of getting something that solves the problem succintly,
         * we'll go with it for now. */
        let normalized_s = input.to_lowercase().replace("?", "");

        /* Here, we split the string by space and turn it into a vector */
        let v: Vec<&str> = normalized_s.split(' ').collect();

        if v[0] != "what" || v[1] != "is" {
            println!("That's not something that \
                      I'm remotely interested in \
                      discussing.");

            continue;
        }

        if v.len() != 5 {
            println!("There don't seem to be the right number of words/numbers...");
            continue;
        }

        /* Get the operator name */
        let operator: &str = v[3];

        /* Make sure we know what it is */
        if !operations.contains_key(operator) {
            println!("Don't know that operation");
            continue;
        }

        /* Get the operands (e.g. [5] for [plus] [5])
         *
         * As before, unwrap applies to a success/failure object and gives
         * the actual value (in this version, we should be error checking,
         * but I'd for demonstration, I'll keep this similar to wordy.rs). */
        let arg1: f64 = v[2].parse::<f64>().unwrap();
        let arg2: f64 = v[4].parse::<f64>().unwrap();

        /* Perform the action using our counter and the operand, arg2 */
        let answer = operations[operator](arg1, arg2);

        println!("Answer is {}", answer);
    }
}
