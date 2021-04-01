/*
https://projecteuler.net/problem=1

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000
*/
fn main() {
    three_or_five(1000);
}

fn three_or_five(max: i32) -> () {
    let mut sum = 0;
    let mut count = 0;
    while count < max - 1 {
        count += 1;
        if count % 3 == 0 || count % 5 == 0 {
            sum += count;
        }
    }
    println!("{}", sum);
}
