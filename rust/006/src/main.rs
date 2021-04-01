/*
https://projecteuler.net/problem=6

The sum of the squares of the first ten natural numbers is,

    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

    (1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is,

    3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main() {
    let nat: Vec<u64> = (1..101).collect();
    let sum_o_sq: u64 = sum_of_squares(&nat);
    let sq_o_sum: u64 = square_of_sums(&nat);
    
    print!("sum_of_squares: {}\n", sum_o_sq);
    print!("square_of_sums: {}\n", sq_o_sum);
    print!("square_of_sums - sum_of_squares = {}", (sq_o_sum - sum_o_sq));
}

fn sum_of_squares(i: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for x in i {
        sum += x * x;
    }
    return sum;
}

fn square_of_sums(i: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for x in i {
        sum += x;
    }
    return sum * sum;
}
