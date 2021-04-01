/*
https://projecteuler.net/problem=5

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let d: Vec<u64> = (2..21).collect();
    let mut n: u64 = 1;
    let mut x: usize = 0;

    while x < d.len() {
        if n % d[x] == 0 {
            x += 1;
        }
        else {
            x = 0;
            n += 1;
        }
    }
    print!("{}\n", n);
}
