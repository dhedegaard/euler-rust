extern crate bit_vec;
use bit_vec::BitVec;
use std::time::SystemTime;

/// Returns a vector of prime numbers up to the cap, the numbers are not sorted in any way.
fn eratosthenes(cap: u64) -> Vec<u64> {
    let cap_sqrt = (cap as f64).sqrt() as u64;
    // Start by allocating a map with all integer values up to cap with a value of true.
    let mut vec = BitVec::from_elem(cap_sqrt as usize, true);
    // For each
    for i in 2..cap_sqrt {
        // Ignore positions already marked as false.
        if !vec.get(i as usize).unwrap() {
            continue;
        }
        // Otherwise, mark positions after this one, step by "i" each time.
        let mut j = i * 2;
        while j < cap_sqrt {
            vec.set(j as usize, false);
            j += i;
        }
    }
    // Take all the true values and aggregate them in a vector.
    let mut result = vec![];
    for key in 2..cap_sqrt {
        if vec.get(key as usize).unwrap() {
            result.push(key)
        }
    }
    result
}

/// Finds and returns the larges primefactor for a given number, if one is found.
fn find_largest_primefactor(number: u64) -> Option<u64> {
    let mut primes = eratosthenes(number);
    // Sort high -> low
    primes.sort_by(|a, b| b.cmp(a));
    for i in primes {
        if number % i == 0 {
            return Some(i);
        }
    }
    None
}

fn main() {
    let before = SystemTime::now();
    match find_largest_primefactor(600851475143) {
        Some(x) => println!("Highest prime factor is: {}", x),
        None => println!("No prime factor found"),
    }
    let duration = SystemTime::now().duration_since(before).unwrap();
    println!(
        "Took: {}.{:03} seconds",
        duration.as_secs(),
        duration.subsec_nanos() / 1_000_000
    );
}
