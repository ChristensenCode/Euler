mod utility;
use std::time::Instant;
use utility::sieve_of_eratosthenes;

///
/// Project Euler Problem #1
///
/// Find the sum of all the multiples of 3 or 5 below 1000
///
pub fn problem_0001() {
    let now = Instant::now();
    let mut final_answer: u32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 {
            final_answer += i;
        } else if i % 5 == 0 {
            final_answer += i;
        }
    }
    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #2
///
/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
///
pub fn problem_0002() {
    let now = Instant::now();
    let mut final_answer: u32 = 0;
    let mut fib_holder = (1, 2);

    while fib_holder.1 < 4_000_000 {
        fib_holder = (fib_holder.1, fib_holder.0 + fib_holder.1);
        if fib_holder.1 % 2 == 0 {
            final_answer += fib_holder.1;
        }
    }
    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #3
///
/// What is the largest prime factor of the number 600851475143?
///
pub fn problem_0003() {
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

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #4
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
pub fn problem_0004() {
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

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #5
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
pub fn problem_0005() {
    let now = Instant::now();
    let mut final_answer: usize = 0;

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

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #6
///
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.
///
pub fn problem_0006() {
    let now = Instant::now();
    let mut final_answer = 0;

    let first_hundred = 1..=100;
    let sum_of_squares: i32 = first_hundred.clone().map(|x: i32| x.pow(2)).sum();
    let square_of_sum: i32 = first_hundred.clone().into_iter().sum();
    final_answer = square_of_sum.pow(2) - sum_of_squares;

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}

///
/// Project Euler Problem #7
///
/// What is the 10,001st prime number?
///
pub fn problem_0007() {
    let now = Instant::now();
    let mut final_answer = 0;

    let primes = sieve_of_eratosthenes(1_000_000);
    final_answer = primes[10_000];

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
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
    let thousand_digit = r#"73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450
"#;
    let mut combined_string = "".to_string();
    for line in thousand_digit.lines() {
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

    println!(
        "The answer is '{}' in {}ms!",
        final_answer,
        now.elapsed().as_millis()
    );
}
