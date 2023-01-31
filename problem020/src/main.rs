fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let mut aa = a;
    let mut bb = b;
    if a >= m {
        aa = a % m;
    }
    if b >= m {
        bb = b % m;
    }
    let c = a * b / m;
    let r = (a*b - c*m) % m;
    if r < 0 { 
        r + m
    } else {
        r
    }
}


fn main() {

}


#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn test_mod() {
        println!("{:?}", (1..30)
            .map(|m| (1..100)
                .fold(1, |a,b| mul_mod(a,b,u64::pow(10,m))))
            .collect::<Vec<u64>>()
        );
    }
    */

    #[test]
    fn test_loop() {
        let p = 52;
        println!("{}", (1..20).map(|x| x % p).product::<u64>());
        println!("{}", (1..20).product::<u64>() % p);

    }
}
