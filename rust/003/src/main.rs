/*
https://projecteuler.net/problem=3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

use primes;

fn main() {
    println!("{}", find_largest(primes::factors(600851475143)));
}

fn find_largest(x :Vec<u64>) -> u64 {
    return x.iter().cloned().fold(2, u64::max);
}
