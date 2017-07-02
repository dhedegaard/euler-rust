struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new = self.curr + self.next;

        self.curr = self.next;
        self.next = new;
        Some(self.curr)
    }
}

fn fibo() -> Fibonacci {
    Fibonacci{curr: 1, next: 0}
}

fn main() {
    let mut sum: u32 = 0;
    for i in fibo() {
        if i > 4_000_000 {
            break;
        }
        if i %2 == 0{
            sum += i;
        }
    }
    println!("{}", sum);
}
