//use itertools::iterate;


/*

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?

*/

fn double_digits(ds : &Vec<u64>) -> Vec<u64> {
    let mut c = 0;
    let mut dds = Vec::new();
    for d in ds { 
        dds.push((2*d + c) % 10);
        c = (2*d + c)/10;
    }
    if c != 0 {
        dds.push(c);
    }
    dds
}



fn main() {

    let n = 1000;

    let ds : Vec<Vec<_>> = itertools::iterate(vec![1], double_digits).into_iter()
        .take(n+1)
        .collect();

    for (i,v) in ds.iter().enumerate() { 
        println!("{} : {}",i, v.iter().sum::<u64>());
    }
}
