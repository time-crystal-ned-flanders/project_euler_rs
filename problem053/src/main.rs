//use std::HashMap;

use itertools::iterate;

/*
fn memchoose() {
    let mut memtab = HashMap<(u128, u128), u128>::new();
    fn choose(n: u128, k: u128) -> Result<u128> {
        if n < 0 || n < k {
            Err("invalid argument")
        } else if n == 0 || k == 0 {
            Ok(1)
        } else {
            Ok(memtab.entry((n,k)).or_insert(choose(n-1, k-1).unwrap() + choose(n-1, k).unwrap()))
        }
    }
    choose
}
*/


fn pascal_triangle_iter(rj: &Vec<u128>) -> Vec<u128> {
    let mut nr = Vec::new();
    if rj.len() <= 0 {
        return vec![1];
    }

    let mut i = 0;
    nr.push(1);
    loop { 
        if i < rj.len()-1 {
            nr.push(rj[i] + rj[i+1]);
        } else { 
            break;
        }
        i += 1;
    }

    nr.push(1);
    nr
}



/*
fn choose(n: u128, k: u128) -> Result<u128> {
    (k+1..n+1).product::<u128>() / (1..n-k+1).product::<u128>()
}
*/



fn main() {
    //println!("{:?}", pascal_triangle_iter(vec![]));
    //println!("{:?}", pascal_triangle_iter(vec![1,1]));
    //println!("{:?}", pascal_triangle_iter(vec![1,2,1]));
    //println!("{:?}", pascal_triangle_iter(vec![1,3,3,1]));

    let ptrows : Vec<Vec<_>> = iterate(vec![1], pascal_triangle_iter).take(101).collect();

    //let ls : Vec<Vec<_>> = (1..100).map(|x| (0..x).map(|y| choose(x,y)).collect()).collect();

    let ngm = ptrows
        .iter()
        .map(|r| r.iter().filter(|x| x > &&1000000).count());

    let mut sum = 0;
    for (i,ngmi) in ngm.enumerate() {
        println!("{}: {}", i, ngmi);
        sum += ngmi;
    }
    println!("total = {}", sum);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_choose() {
        println!("")
    }
}
