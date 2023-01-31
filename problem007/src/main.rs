fn main() {
    let n = 10001;
    let mut k = 3;
    let mut primes = vec![2];
    while primes.len() < n {
        if !primes.iter().any(|p| k % p == 0) {
            primes.push(k);
        }   
        k = k+2;
    }
    println!("{:?}", primes);
    println!("the {}th prime is: {}", n, primes[n-1])
}


