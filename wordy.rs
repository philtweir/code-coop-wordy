/**
 * wordy
 *
 * A partial solution to http://exercism.io/exercises/rust/wordy/readme
 *
 * Can accept queries of the form:
 *      What is NUM1 OP1 NUM2 OP2...?
 *  where each OP is an operator of one or more words.
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
    operations.insert("divided by", Box::new(|x: f64, y: f64| x / y));
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

        /* Now... because of 'divided by', operators can be multiple
         * words. That means, we really want sections like:
         * [WORDS] [NUMBER] [WORDS] [NUMBER] [WORDS] [NUMBER]...
         * [What is] [3] [divided by] [19[ [plus] [12]
         *
         * We call each of these a segment.
         *
         * I'd probably do this pairwise (WORDS+NUMBER), but to avoid
         * the code getting more confusing, we'll just create a single
         * list of all the segments, WORDS and NUMBERS
         */
        let mut segments = Vec::new();

        /* We will use this flag to check for alternation and round out a segment */
        let mut on_a_number = false;

        /* As our segments consist of (potentially) multiple words, we need to hold them together
         * until we have a whole segment to put in our segments vector */
        let mut this_segment = Vec::new();

        /* Go through each space-separated token... */
        for token in &v {
            /* Try and parse it as a float... */
            let component = &token.parse::<f64>();

            /* If we got a float OK and the preceding segment is a word
             * or we didn't and the preceding segment is a number,
             * we've got a segment boundary. Slash and stash! */
            if component.is_err() == on_a_number {
                /* Join this segment together, e.g. ["What", "is"] --> "What is" */
                let segment_str = this_segment.join(" ");

                /* Add it to our segments list */
                segments.push(segment_str);

                /* Flip the flag, as the next word should be the opposite */
                on_a_number = !on_a_number;

                /* Sweep the floor clean. This is now a new, empty segment */
                this_segment.clear();
            }

            /* Stick the current token into the current segment */
            this_segment.push(token.to_string());
        }
        /* Our last segment isn't closed off by a change to/from a number, so
         * we manually add it */
        segments.push(this_segment.join(" "));
        println!("[{}]", segments.join("] ["));

        if segments.len() < 4 {
            println!("There don't seem to be enough numbers...");
            continue;
        }

        if segments.len() % 2 != 0 {
            println!("Numbers and operators should alternate");
            continue;
        }

        /* We work out how many [WORDS]+[NUMBER] sets there are and loop
         * through them */
        let actions = (segments.len() - 1) / 2 + 1;

        /* We need to have some counter as we move through that contains
         * the current sum. It starts out as the leftmost number.
         *
         * As before, unwrap applies to a success/failure object and gives
         * the actual value (we know parsing should be successful by our
         * segment construction process) */
        let mut accumulator: f64 = segments[1].parse::<f64>().unwrap();

        for action in 1..actions {
            /* Get the name of the operator (e.g. [plus] for [plus] [5]) */
            let operator: &str = segments[2 * action].as_str();

            /* Make sure we know what it is */
            if !operations.contains_key(operator) {
                println!("Don't know that operation");
                continue;
            }

            /* Get the right hand operand (e.g. [5] for [plus] [5]) */
            let arg2: f64 = segments[2 * action + 1].parse::<f64>().unwrap();

            /* Perform the action using our counter and the operand, arg2 */
            accumulator = operations[operator](accumulator, arg2);
        }

        println!("Answer is {}", accumulator);
    }
}
