fn main() {
    let mut sum = 0;
    for k in fib() {
        if k >= 4000000 {
            break;
        }
        if k % 2 != 0 {
            continue;
        }
        sum += k;
    }
    println!("{}",sum);
}

struct Fib {
    curr : u64,
    next : u64
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.curr;
        self.curr = self.next;
        self.next = self.next + c;
        Some(c)
    }
}

fn fib() -> Fib {
    Fib { curr: 0, next: 1 }
}
