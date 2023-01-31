//use itertools::Itertools;
use itertools::iproduct;

pub fn digit_vec(n : u64) -> Vec<u64> {
    let mut ds = Vec::new();
    let mut nn = n;
    while nn > 1 {
        ds.push(nn % 10);
        nn = nn / 10;
    }
    if nn % 10 != 0 {
        ds.push(nn % 10);
    }

    //println!("{:?}",ds);
    ds.reverse();
    //println!("{:?}",ds);
    ds
}

pub fn is_palindrome(ds : Vec<u64>) -> bool {
    ds.iter().eq(ds.iter().rev())
}

// print out i * j in descending order
// i*j, i-1, j

fn main() { 
    let mut ts : Vec<(u64, u64, u64)>= iproduct!((0..1000).rev(), (0..1000).rev())
        .map(|(i,j)| (i*j,i,j))
        .filter(|(n,i,j)| is_palindrome(digit_vec(n.clone())))
        .collect();
    
    ts.sort_by_key(|k| k.0);
    println!("{:?}", ts);
    

}

/*

fn main() {
    println!("Largest palindromic number from prod of 3 digit numbers");
    'out: for i in (0..1000).rev() { 
        for j in (0..1000).rev() {
            let n = i * j;
            println!("processing {} = {} * {}", n, i, j);
            let dv = digit_vec(n);
            if is_palindrome(dv) {
                println!("{} * {} = {}", i, j, i*j);
                break 'out;
            }
        }
    }


}

*/


#[cfg(test)]
mod test { 
    use super::*;
    #[test]
    fn test_digit_vec() {
        assert_eq!(digit_vec(998001), vec![9, 9, 8, 0, 0, 1]);
    }
    #[test]
    fn test_is_palindrome1() { 
        assert_eq!(is_palindrome(vec![1,2,1]), true);
    }
    #[test]
    fn test_is_palindrome2() { 
        assert_eq!(is_palindrome(digit_vec(7123217)), true);
    }


}

