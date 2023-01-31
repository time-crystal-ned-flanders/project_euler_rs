fn main() {
    println!("lcm(1..11) = {}", lcm((1..11).collect()));
    println!("lcm(1..21) = {}", lcm((1..21).collect()));
}

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
mod test { 
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(1, 18), 1);

    }
    #[test]
    fn test_lcm() { 
        assert_eq!(lcm(vec![2,3,4]), 12);
    }
}

