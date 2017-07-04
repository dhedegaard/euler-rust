extern crate bit_vec;
use bit_vec::BitVec;

/// Returns a vector of prime numbers up to the cap, the numbers are not sorted in any way.
fn eratosthenes(cap: u64) -> BitVec {
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
    vec
}

/// Finds and returns the larges primefactor for a given number, if one is found.
fn find_largest_primefactor(number: u64) -> Option<u64> {
    let primes = eratosthenes(number);
    // Iterate high -> low
    println!("len: {}", primes.len());
    match (2..primes.len())
        .rev()
        .filter(|e| primes[*e] && number % (*e as u64) == 0)
        .next() {
        Some(x) => Some(x as u64),
        None => None,
    }
}

fn main() {
    match find_largest_primefactor(600851475143) {
        Some(x) => println!("{}", x),
        None => println!("No prime factor found"),
    }
}
