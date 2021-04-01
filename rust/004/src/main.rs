/*
https://projecteuler.net/problem=4

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit number
*/
// use std::collections::VecDeque;

fn main() {
    let queue: Vec<u64> = (100..1000).collect();
    let mut count: usize = 0;
    let mut pal: Vec<u64> = Vec::new();

    while count < queue.len() {
        for x in &queue {
            let prod: u64 = &queue[count] * x;
            if is_palindrome(&prod.to_string()) {
                pal.push(prod);
            }
        }
        count += 1;
    }

    print!("{}\n", find_largest(pal));
}

fn is_palindrome(s: &str) -> bool {
    return s == s.chars().rev().collect::<String>();
}

fn find_largest(x: Vec<u64>) -> u64 {
    return x.iter().cloned().fold(1, u64::max);
}
