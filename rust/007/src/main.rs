/*
https://projecteuler.net/problem=7

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

use primal;

fn main() {
    let nth: u64 = 10001;
    let (_lo, hi) = primal::estimate_nth_prime(nth);
    let sieve = primal::Sieve::new(hi as usize);

    println!("The 10'001st prime is {}", sieve.nth_prime(nth as usize));
}
