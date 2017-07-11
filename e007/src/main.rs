fn is_prime(num: u32) -> bool {
    if num < 4 {
        return true;
    }
    for i in 2..num / 2 + 1 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut primes_encountered = 0;
    let mut i = 2;
    loop {
        if is_prime(i) {
            primes_encountered += 1;
        }
        if primes_encountered == 10_001 {
            break;
        }
        i += 1;
    }
    println!("{}", i);
}
