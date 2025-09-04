use std::time::Instant;

use num2words::Num2Words;
use num_bigint::BigUint;

use crate::utility::utility::output_statement;

/// Project Euler Problem #16
/// What is the sum of the digits of the number 2 ^ 1000?
pub fn problem_0016() {
    let now = Instant::now();
    let base = BigUint::from(2u32);
    let exponent = 1000u32;
    let final_answer: i32 = base
        .pow(exponent)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .sum();

    output_statement("Problem 0016", final_answer.to_string(), now);
}

/// Project Euler Problem #17
/// If all the numbers from 1 to 1000 (one thousand)
/// inclusive were written out in words, how many letters would be used?
pub fn problem_0017() {
    let now = Instant::now();
    // Starting point accounting for all the missing 'and' values.
    let mut final_answer = 3 * 901;

    for i in 1..1000 {
        let words = Num2Words::new(i)
            .to_words()
            .unwrap()
            .replace(' ', "")
            .replace('-', "")
            .len();
        final_answer += words;
    }
    output_statement("Problem 0017", final_answer.to_string(), now);
}

/// Project Euler Problem #18
/// Find the maximum total from top to bottom of the triangle below:
pub fn problem_0018() {
    let thing = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    println!("{}", thing)
}

/// Project Euler Problem #19
/// How many Sundays fell on the first of the month during the
/// twentieth century (1 Jan 1901 to 31 Dec 2000)?
pub fn problem_0019() {
    println!("thing")
}

/// Project Euler Problem #20
/// Find the largest sum of digits in the number 100!
pub fn problem_0020() {
    println!("thing")
}

/// Project Euler Problem #21
/// Evaluate the sum of all the amicable numbers under 10,000
pub fn problem_0021() {
    println!("thing")
}

/// Project Euler Problem #22
/// What is the total of all the name scores in the file?
pub fn problem_0022() {
    println!("thing")
}

/// Project Euler Problem #23
/// Find the sum of all the positive integers which cannot
/// be written as the sum of two abundant numbers.
pub fn problem_0023() {
    println!("thing")
}

/// Project Euler Problem #24
/// What is the millionth lexicographic permutation of the
/// digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
pub fn problem_0024() {
    println!("thing")
}

/// Project Euler Problem #25
/// What is the index of the first term in the Fibonacci
/// sequence to contain 1,000 digits?
pub fn problem_0025() {
    println!("thing")
}

/// Project Euler Problem #26
/// Find the value of d < 1,000 for which 1/d contains the
/// longest recurring cycle in its decimal fraction part.
pub fn problem_0026() {
    println!("thing")
}

/// Project Euler Problem #27
/// Find the product of the coefficients, a and b, for the
/// quadratic expression that produces the maximum number of
/// primes for consecutive values of n, starting with n=0.
pub fn problem_0027() {
    println!("thing")
}

/// Project Euler Problem #28
/// What is the sum of the numbers on the diagonals in a 1,001
/// by 1,001 spiral formed in the same way?
pub fn problem_0028() {
    println!("thing")
}

/// Project Euler Problem #29
/// How many distinct terms are in the sequence generated
/// by a^b for 2 <= a <= 100 and 2 <= b <= 100?
pub fn problem_0029() {
    println!("thing")
}

/// Project Euler Problem #30
/// Find the sum of all the numbers that can be written
/// as the sum of fifth powers of their digits.
pub fn problem_0030() {
    println!("thing")
}
