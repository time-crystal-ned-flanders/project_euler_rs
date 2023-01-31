// euclidean gcd algo
pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn lcm(xs: Vec<u64>) -> u64 {
    xs.into_iter().fold(1, |a,b| a*b/gcd(a,b))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
