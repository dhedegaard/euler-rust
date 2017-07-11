fn main() {
    let mut num = 20;
    loop {
        let mut success = true;
        for i in 3..20 {
            if num % i != 0 {
                success = false;
                num += 20;
                break;
            }
        }
        if success {
            println!("{}", num);
            break;
        }
    }
}
