/*
https://projecteuler.net/problem=7

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

use slow_primes;

fn main() {
    let nth: u64 = 10001;
    let (_lo, hi) = slow_primes::estimate_nth_prime(nth);
    let sieve = slow_primes::Primes::sieve(hi as usize);

    match sieve.primes().nth((nth as usize) - 1) {
        Some(p) => println!("The 10001st prime is {}", p),
        None => unreachable!(),
    }
}
