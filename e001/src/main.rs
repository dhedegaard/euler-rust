fn main() {
    println!("{}", (1..1000).filter(|e| e % 3 == 0 || e % 5 == 0).fold(0, |acc, e| { acc + e }));
}
