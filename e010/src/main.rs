extern crate bit_vec;
use bit_vec::BitVec;

fn eratosthenes(cap: u32) -> BitVec {
    let mut vec = BitVec::from_elem(1 + cap as usize, true);
    vec.set(0, false);
    vec.set(1, false);

    for i in 2..1 + (cap as f64).sqrt() as usize {
        if !vec[i] {
            continue;
        }
        for j in 2.. {
            if i * j > cap as usize {
                break;
            }
            vec.set(i * j, false);
        }
    }
    vec
}

fn main() {
    let primes = eratosthenes(2_000_000);
    let result = primes
        .iter()
        .enumerate()
        .filter(|&(_, e)| e)
        .map(|(idx, _)| idx)
        .fold(0, |acc, e| acc + e);
    println!("{}", result);
}
