// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false
    }
    let mut j = 2;
    while j*j <= n {
        if n % j == 0 {
            return false
        }
        j+=1;
    }
    true
}
//2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97.

fn main() {
    let N = 2000000;
    let mut sum = 0;
    let mut primes = Vec::new();
    for n in 1..N {
        if is_prime(n) {
            primes.push(n);
            sum += n;
        }
    }
    println!("primes: {:?}", primes);
    println!("sum of primes < {} = {}", N, sum);
}
