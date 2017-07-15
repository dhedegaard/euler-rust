fn main() {
    for c in 1..1000 {
        for b in 1..c {
            for a in 1..b {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    println!("{}", a * b * c);
                    std::process::exit(0);
                }
            }
        }
    }
}
