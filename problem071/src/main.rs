use std::cmp::Ordering;

pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

// different algo  we want largest fraction n/d upper bounded by 3/7
// find largest fraction < 3/7 with 8 >

fn reduce_frac((n,d):(u64,u64)) -> (u64,u64) {
    let div = gcd(n,d);
    (n/div, d/div)
}

fn is_red_prop((n,d):(u64,u64)) -> bool {
    n < d && gcd(n,d) > 1
}

fn cmp_frac((a,b):(u64,u64), (c,d):(u64,u64)) -> std::cmp::Ordering {
    (a*d).cmp(&(b*c))
}

fn main() {
    let dmax = 1000000;
    let tar = (3,7);
    let mut lesstar : Vec<_> = (1..dmax)
        .map(|d| (tar.0*d/tar.1,d))
        .map(|f| reduce_frac(f.clone()))
        .filter(|f| *f != tar)
        .collect();
    lesstar.sort_by(|a,b| cmp_frac(*a,*b));
    println!("{lesstar:?}");
}

