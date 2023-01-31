#![feature(map_first_last)]

use std::collections::{HashSet, BTreeSet, BTreeMap};



fn test_square(n : u128) -> Option<u128> {
    let mut left = 1;
    let mut right = n;
    while left <= right {
        let mid = (left + right)/2;
        if mid*mid == n {
            return None;
        }
        if mid*mid < n {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    } 
    return Some(right);
}

fn convergent_pell(d: u128, n: u128) ->  (u128, u128) { 

}

fn cf_sqrt(n: u128) -> Option<vec<u128>> {
    if let Some(n0) = test_square(n) {
        let (x,y,z) = (0,1,1);
        let mut a = Vec::new();
        fn step(x: u128, y: u128, z: u128) -> (u128, u128, u128) {
            a.push((x*n0 + y)/z);
            let t = y - a.last()*z;
            (x,y,z) = z*t, -z*y, t*t - n*x*x
            g
        }

    } else {
        None
    }
}




fn main() {




}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sq() {
        println!("{:?}", test_square(4));
        println!("{:?}", test_square(16));

        println!("{:?}", test_square(17));
        println!("{:?}", test_square(15));
        println!("{:?}", test_square(257));

    }


    
}