use std::time::Instant;

/// Finds PRIME numbers.
// #[allow(dead_code)]
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut prime: Vec<bool> = vec![true; n + 1];
    let mut found_primes: Vec<usize> = Vec::new();

    let mut p: usize = 2;

    while p * p <= n {
        if prime[p] == true {
            for i in (p * p..n + 1).step_by(p) {
                prime[i] = false;
            }
        }

        p += 1;
    }

    for value in 2..n + 1 {
        if prime[value] {
            found_primes.push(value);
        }
    }

    found_primes
}

/// Brief.
///
/// Description.
///
/// * `problem_number` - Text about foo.
/// * `calculated_answer` - Text about bar.
/// * `computation_time` - Text about bar.
pub fn output_statement(
    problem_number: &str,
    calculated_answer: String,
    computation_time: Instant,
) {
    let elapsed_time = computation_time.elapsed().as_millis();

    println!(
        "{} - The answer is '{}' in {}ms!",
        problem_number, calculated_answer, elapsed_time
    );
}
