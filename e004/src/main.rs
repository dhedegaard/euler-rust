fn main() {
    let mut largest_result = -1;
    for i in (800..1000).rev() {
        for j in (800..1000).rev() {
            let result = format!("{}", i * j);
            if result == result.chars().rev().collect::<String>() {
                let result = i * j;
                if largest_result < result {
                    largest_result = result;
                }
            }
        }
    }
    println!("{}", largest_result);
}
