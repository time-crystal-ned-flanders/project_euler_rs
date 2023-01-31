//The cube, 41063625 (3453), can be permuted to produce two other cubes: 56623104 (3843) and 66430125 (4053). In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.

//Find the smallest cube for which exactly five permutations of its digits are cube.

use std::collections::HashMap;

fn digits(n:u64) -> Vec<u64> {
    // map integer to a 10 digit string which counts the number of times that each digit
    // 0-9 occurs two numbers have the same  
    let mut np = n;
    let mut digits : Vec::<u64> = Vec::<u64>::new();
    while np != 0 {
        digits.push(np % 10);
        np /= 10;
    }
    digits.reverse();
    digits
}

fn digitcount(n:u64) -> [u64; 10] {
    let mut np = n;
    let mut dc : [u64; 10] = [0; 10];
    while np != 0 {
        dc[(np % 10) as usize] += 1;
        np /= 10;
    }
    dc
}


fn main() {
    let mut hs : HashMap<[u64; 10], Vec<u64>> = HashMap::new();
    
    let nmax = 2000000;
    let mut mincube5perm = nmax;
        
    for n in 1..nmax {
        let n3 = digitcount(n*n*n);
        let nl = hs.entry(n3).and_modify(|ls| ls.push(n)).or_insert(vec![n]);           
        
        if nl.len() == 5 {
            let m = nl.iter().min().unwrap();
            if m < &mincube5perm {
                mincube5perm = *m;
            }    
        }
    }
    println!("{} - {}", mincube5perm, mincube5perm*mincube5perm*mincube5perm);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_1a() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_digicount() {
        for n in 0..50 {
            println!("{} = {:?}", n, digitcount(n));
        }
    }


    #[test]
    fn test_hm() {
        let mut hm : HashMap<Vec<u64>, u64> = HashMap::new();
        for k in 1..20 {
            hm.entry( digits( k*k*k ) ).and_modify(|count| *count += 1).or_insert(0);
        }
        println!("{:?}", hm);
        assert_eq!(true, true);
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![1,2,3], digits(123));
        assert_eq!(vec![1], digits(1));
        assert_eq!(Vec::<u64>::new(), digits(0));
        assert_eq!(vec![5,7,8,9,1], digits(57891));
        assert_eq!(digits(145), digits(145));
    }
}
