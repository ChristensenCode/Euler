use crate::utility::utility::{output_statement, sieve_of_eratosthenes};
use std::{env, fs, time::Instant};

///
/// Project Euler Problem #1
///
/// Find the sum of all the multiples of 3 or 5 below 1000
///
pub fn problem_0001() -> u32 {
    let now = Instant::now();
    let mut final_answer: u32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 {
            final_answer += i;
        } else if i % 5 == 0 {
            final_answer += i;
        }
    }
    output_statement("Problem 0001", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #2
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
///
pub fn problem_0002() -> u32 {
    let now = Instant::now();
    let mut final_answer: u32 = 0;
    let mut fib_holder = (1, 2);

    while fib_holder.1 < 4_000_000 {
        fib_holder = (fib_holder.1, fib_holder.0 + fib_holder.1);
        if fib_holder.1 % 2 == 0 {
            final_answer += fib_holder.1;
        }
    }
    output_statement("Problem 0002", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #3
///
/// What is the largest prime factor of the number 600851475143?
///
pub fn problem_0003() -> usize {
    let now = Instant::now();

    // let final_answer: Vec<bool> = sieve_of_eratosthenes(100);
    let list_of_primes: Vec<usize> = sieve_of_eratosthenes(775146);
    let mut final_answer: usize = 0;

    for i in list_of_primes.iter().rev() {
        if 600851475143 % i == 0 {
            final_answer = *i;
            break;
        }
    }
    output_statement("Problem 0003", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #4
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
pub fn problem_0004() -> usize{
    let now = Instant::now();
    let mut final_answer: usize = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let possible_answer = i * j;
            let as_string: String = possible_answer.to_string();
            let as_string_rev: String = as_string.chars().rev().collect();
            if as_string_rev == as_string && possible_answer > final_answer {
                final_answer = possible_answer;
            }
        }
    }
    output_statement("Problem 0004", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #5
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
pub fn problem_0005() -> usize{
    let now = Instant::now();
    let final_answer: usize;

    let mut step_size = 1;
    for i in 17..=20 {
        step_size *= i;
    }

    // let mut counter = 19 * 20;
    let mut counter = step_size;
    loop {
        let mut all_divisible = 1;
        for i in 2..=20 {
            if counter % i == 0 {
                all_divisible += 1;
            } else {
                break;
            }
        }
        if all_divisible == 20 {
            final_answer = counter;
            break;
        }

        // counter += 20 * 19;
        counter += step_size;
    }

    output_statement("Problem 0005", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #6
///
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.
///
pub fn problem_0006() -> i32 {
    let now = Instant::now();

    let first_hundred = 1..=100;
    let sum_of_squares: i32 = first_hundred.clone().map(|x: i32| x.pow(2)).sum();
    let square_of_sum: i32 = first_hundred.clone().into_iter().sum();
    let final_answer = square_of_sum.pow(2) - sum_of_squares;

    output_statement("Problem 0006", final_answer.to_string(), now);
    return final_answer;
}

///
/// Project Euler Problem #7
///
/// What is the 10,001st prime number?
///
pub fn problem_0007() {
    let now = Instant::now();

    let primes = sieve_of_eratosthenes(1_000_000);
    let final_answer = primes[10_000];
    output_statement("Problem 0007", final_answer.to_string(), now);
}

///
/// Project Euler Problem #8
///
/// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
/// What is the value of this product?
///
pub fn problem_0008() {
    let now = Instant::now();
    let mut final_answer = 0;
    let location = env::current_dir();
    let data_path = format!("{}/src/data/problem_0008.txt", location.expect("REASON").display());
    let contents = fs::read_to_string(data_path).expect("The data should be read in!");
    let mut combined_string = "".to_string();
    for line in contents.lines() {
        combined_string += line;
    }

    for i in 0..=1000 - 13 {
        let slicer = &combined_string[i..i + 13];
        if slicer.contains("0") {
            continue;
        }

        let mut individual_values = Vec::new();
        for ch in slicer.chars() {
            if let Some(digit) = ch.to_digit(10) {
                individual_values.push(digit as u64);
            }
        }

        let combined_value = individual_values.iter().fold(1, |acc, &x| acc * &x);

        if combined_value > final_answer {
            final_answer = combined_value;
        }
    }

    output_statement("Problem 0008", final_answer.to_string(), now);
}

///
/// Project Euler Problem #9
///
/// There exists exactly one Pythagorean triplet for which a+b+c=1000.
/// Find the product a*b*c (here)
///
pub fn problem_0009() {
    let now = Instant::now();
    let mut final_answer = 0.0;

    'outer: for a in 1..=1000 {
        let a: f32 = a as f32;
        for b in 1..=1000 {
            let b: f32 = b as f32;
            let c = (a.powf(2.0) + b.powf(2.0)).sqrt();
            if c.fract() == 0.0 && a + b + c == 1000.0 {
                final_answer = a * b * c;
                break 'outer;
            }
        }
    }
    output_statement("Problem 0009", final_answer.to_string(), now);
}

///
/// Project Euler Problem #10
///
/// Find the sum of all primes below two million
///
pub fn problem_0010() {
    let now = Instant::now();

    let primes = sieve_of_eratosthenes(2_000_000);
    let final_answer: usize = primes.iter().sum();
    output_statement("Problem 0010", final_answer.to_string(), now);
}

///
/// Project Euler Problem #11
///
/// Calculate the largest product of four adjacent numbers.
///
pub fn problem_0011() {
    let now = Instant::now();
    let mut final_answer = 0;
    let location = env::current_dir();
    let data_path = format!("{}/src/data/problem_11.txt", location.expect("REASON").display());
    let contents = fs::read_to_string(data_path).expect("The data should be read in!");
    let line_data = contents.lines();

    let mut combined_line = vec![];

    for line in line_data {
        let values: Vec<&str> = line.split(' ').collect();
        combined_line.extend(values);
    }

    let all_values: Vec<usize> = combined_line
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let target_value = 0;

    let mut row_length = 0;
    let hor_line = vec![0, 1, 2, 3];
    for _ in 0..20 {
        for width in 0..=16 {
            let hor_locations: Vec<usize> =
                hor_line.iter().map(|&x| x + width + row_length).collect();
            let indexed_values: Vec<_> = hor_locations.iter().map(|&i| all_values[i]).collect();
            if indexed_values.contains(&target_value) {
                continue;
            }
            let possible_value: i32 = indexed_values.iter().fold(1, |acc, &x| acc * x as i32);
            if possible_value > final_answer {
                final_answer = possible_value;
            }
        }
        row_length += 20;
    }

    let mut row_length = 0;
    let vert_line = vec![0, 20, 40, 60];
    for _ in 0..16 {
        for width in 0..=20 {
            let vert_locations: Vec<usize> =
                vert_line.iter().map(|&x| x + width + row_length).collect();
            let indexed_values: Vec<_> = vert_locations.iter().map(|&i| all_values[i]).collect();
            if indexed_values.contains(&target_value) {
                continue;
            }
            let possible_value: i32 = indexed_values.iter().fold(1, |acc, &x| acc * x as i32);
            if possible_value > final_answer {
                final_answer = possible_value;
            }
        }
        row_length += 20;
    }

    let mut row_length = 0;
    let diag_down_line = vec![0, 21, 42, 63];
    for _ in 0..=16 {
        for width in 0..=16 {
            let diag_down_locations: Vec<usize> = diag_down_line
                .iter()
                .map(|&x| x + width + row_length)
                .collect();
            let indexed_values: Vec<_> =
                diag_down_locations.iter().map(|&i| all_values[i]).collect();
            if indexed_values.contains(&target_value) {
                continue;
            }
            let possible_value: i32 = indexed_values.iter().fold(1, |acc, &x| acc * x as i32);
            if possible_value > final_answer {
                final_answer = possible_value;
            }
        }
        row_length += 20;
    }

    let mut row_length = 0;
    let diag_up_line = vec![60, 41, 22, 3];
    for _ in 0..=16 {
        for width in 0..=16 {
            let diag_up_locations: Vec<usize> = diag_up_line
                .iter()
                .map(|&x| x + width + row_length)
                .collect();
            let indexed_values: Vec<_> = diag_up_locations.iter().map(|&i| all_values[i]).collect();
            if indexed_values.contains(&target_value) {
                continue;
            }
            let possible_value: i32 = indexed_values.iter().fold(1, |acc, &x| acc * x as i32);
            if possible_value > final_answer {
                final_answer = possible_value;
            }
        }
        row_length += 20;
    }

    output_statement("Problem 0011", final_answer.to_string(), now);
}

///
/// Project Euler Problem #12
///
///
/// What is the value of the first triangle number
/// to have over five hundred divisors?
///
pub fn problem_0012() {
    let now = Instant::now();

    let mut natural_number = 2;
    let mut number_of_divisors = 0;
    let mut triangle_number;
    let mut divisors = 1;

    loop {
        triangle_number = (1..natural_number + 1).sum();
        while divisors * divisors <= triangle_number {
            if triangle_number % divisors == 0 {
                number_of_divisors += 1;
                let floored = triangle_number / divisors;
                if (floored as i32) != divisors {
                    number_of_divisors += 1;
                }
            }
            divisors += 1;
        }

        if number_of_divisors > 500 {
            break;
        }

        natural_number += 1;
        number_of_divisors = 0;
        divisors = 1;
    }

    let final_answer = triangle_number;
    output_statement("Problem 0012", final_answer.to_string(), now);
}

///
/// Project Euler Problem #13
///
/// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
///
pub fn problem_0013() {
    let now = Instant::now();
    let location = env::current_dir();
    let data_path = format!("{}/src/data/problem_13.txt", location.expect("REASON").display());
    let contents = fs::read_to_string(data_path).expect("The data should be read in!");
    let line_data = contents.lines();

    let mut intermediate_answer = 0.0;

    for line in line_data {
        let x: f64 = line.parse().unwrap();
        intermediate_answer += x;
    }

    let almost_answer = intermediate_answer.to_string();
    let final_answer = &almost_answer[..10];
    output_statement("Problem 0013", final_answer.to_string(), now);
}

///
/// Project Euler Problem #14
///
/// Which starting number, under one million, produces the longest chain?
///
pub fn problem_0014() {
    let now = Instant::now();

    let mut final_answer = 0;
    let mut longest_chain = 0;

    for possible_answer in 113000..=1_000_000 {
        let mut n: i64 = possible_answer;
        let mut chain_length = 0;
        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1
            }
            chain_length += 1;
        }

        if chain_length > longest_chain {
            longest_chain = chain_length;
            final_answer = possible_answer;
        }
    }
    output_statement("Problem 0014", final_answer.to_string(), now);
}

///
/// Project Euler Problem #15
///
/// How many such routes are there through a 20x20 grid?    
///
pub fn problem_0015() {
    let now = Instant::now();

    pub fn factorial(input_number: i32) -> f64 {
        let mut answer = input_number as f64;
        for i in 1..input_number {
            answer *= i as f64;
        }
        answer
    }

    let top = factorial(40);
    let bottom = factorial(20) * factorial(20);
    let final_answer = (top / bottom) as i64;

    output_statement("Problem 0015", final_answer.to_string(), now);
}
