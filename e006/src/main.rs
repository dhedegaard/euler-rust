fn main() {
    let square_of_sum = (1..101).fold(0, |acc, e| acc + e);
    let sum_of_squares = (1..101).map(|e| e * e).fold(0, |acc, e| acc + e);
    println!("{}", square_of_sum * square_of_sum - sum_of_squares);
}
