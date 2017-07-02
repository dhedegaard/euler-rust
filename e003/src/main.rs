use std::collections::HashMap;

/// Returns a vector of prime numbers up to the cap, the numbers are not sorted in any way.
fn eratosthenes(cap: u64) -> Vec<u64> {
    let cap_sqrt = (cap as f64).sqrt() as u64;
    // Start by allocating a map with all integer values up to cap with a value of true.
    let mut map = HashMap::with_capacity(cap_sqrt as usize);
    for i in 2..cap_sqrt {
        map.insert(i, true);
    }
    // For each 
    for i in 2..cap_sqrt {
        // Ignore positions already marked as false.
        if !map.get(&i).unwrap() {
            continue
        }
        // Otherwise, mark positions after this one, step by "i" each time.
        let mut j = i * 2;
        while j < cap_sqrt {
            map.insert(j, false);
            j += i;
        }
    }
    // Take all the true values and aggregate them in a vector.
    let mut result = vec!();
    for (key, value) in &map {
        if *value {
            result.push(*key)
        }
    }
    println!("ERA END");
    result
}

/// Finds and returns the larges primefactor for a given number, returns None if no number was found.
fn find_largest_primefactor(number: u64) -> Option<u64> {
    let mut primes = eratosthenes(number);
    // Sort high -> low
    primes.sort_by(|a, b| b.cmp(a));
    for i in primes {
        if number % i == 0 {
            return Some(i)
        }
    }
    None
}

fn main() {
    match find_largest_primefactor(600851475143) {
        Some(x) => println!("Highest prime factor is: {}", x),
        None    => println!("No prime factor found"),
    }
}
