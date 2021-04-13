/*
https://projecteuler.net/problem=1

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

package main

import "fmt"

func main() {
	threeOrFive(1000)
}

func threeOrFive(i int) {
	var sum int = 0
	var count int = 0
	for count < i-1 {
		count += 1
		if count%3 == 0 || count%5 == 0 {
			sum += count
		}
	}
	fmt.Printf("%d", sum)
}
