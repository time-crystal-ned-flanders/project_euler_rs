use std::fs::{File};
use std::io::{self, BufRead, Result};
use std::iter;
use itertools::Itertools;


fn load_triangle(filename: &str) -> Result<Vec<Vec<u64>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut vv : Vec<Vec<u64>> = Vec::new();
    for line in reader.lines().filter_map(Result::ok) {
        let mut ll : Vec<u64> = Vec::new();
        for tok in line.split_whitespace() {
            ll.push(tok.parse::<u64>().unwrap())
        }
        vv.push(ll);
    }
    Ok(vv)
}

// algorithm start at the bottom row and next to bottom row
/*
fn construct_path(rows: Vec<Vec<u64>>) -> u64 {
}
*/

fn zero_pad(a: Vec<u64>) -> Vec<u64> {
    iter::once(0).chain(a.into_iter()).chain(iter::once(0)).collect()
}

fn merge_rows(a: Vec<u64>, b: Vec<u64>) -> Vec<u64> { 
    itertools::zip(zero_pad(a).windows(2), b)
        .map(|(v, a)| a + v.iter().max().unwrap())
        .collect()
}


fn main() -> Result<()> {
    //let filename = "p018.txt";
    let filename = "p067_triangle.txt";
    
    let mut vv : Vec<Vec<_>> = load_triangle(filename)?;

    println!("");
    vv.iter().for_each(|v| println!("{:?}",v));

    //println!("{:?}", merge_rows(vv[0].clone(), vv[1].clone()));

    let vvp = vv.into_iter().fold1(merge_rows);

    println!("{:?}", vvp.unwrap().iter().max() );

    Ok(())
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn t1() {
        let mut w : Vec<u64> = vec![4,5,6];
        let wp : Vec<_> = iter::once(&0).chain(w.iter()).chain(iter::once(&0)).collect();
        //let wp : Vec<_> = w.iter().chain(iter::once(&0)).collect();
        println!("{:?} {:?}", w, wp);
    }


    #[test]
    fn test_tw() { 
        for w in itertools::zip(vec![1,2,3,4,5], vec![1,2,3,4,5,6].windows(2)) {
            println!("{:?}", w);        
        }
        println!("{:?}", vec![1,2,3,4,5,6].windows(2));
    }

    #[test]
    fn test_it() { 
        println!("test it");
        let z : Vec<_> = itertools::zip(vec![1,2,3,4,5], vec![1,2,3,4,5,6].windows(2))
            .map(|(a, v)| a + v.iter().max().unwrap())
            .collect();
        println!("{:?}",z);
    }

    #[test]
    fn test_row() { 
        let tv = merge_rows(vec![1,2,3,4], vec![2,5,2]);
        println!("tv - {:?}", tv );
    }
}