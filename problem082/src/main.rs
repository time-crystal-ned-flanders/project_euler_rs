use std::fs::{File};
use std::io::{self, BufRead, Result};
use std::convert::TryInto;

fn min_map(v: Vec<u32>, vn: Vec<u32>) -> Vec<u32> {
    let vl = v.len();
    let mut result = vec![0; vl];
    println!("{:?}", vn);
    println!("{:?}", v);
    for i in 0..vl {
        let mut cpath = vec![0; vl];
        for j in 0..vl {
            let (start, stop) = (usize::min(i,j), usize::max(i,j));
            println!("i={i:} j={j:} start={start:} stop={stop:}");

            cpath[j] = v.iter().skip(start).take(stop).sum::<u32>() + v[start] + vn[stop];

            println!("v[start]={} v[stop]={} vn[j]={} len={}", v[start], v[stop], vn[j], cpath[j]);

        }
        println!("--------");
        result[i] = cpath.into_iter().min().unwrap();
    }
    result
}


// transpose in place Vec<Vec<T>> - copied from stackoverflow - thanks user4815162342 Netwave
fn transpose(v: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn spath(v: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut res : Vec<Vec<_>> = Vec::new();
    let vl = v.len();
    res.push(v[vl-1].clone());
    for i in (0..vl-1).rev() {
        res.push(min_map(v[i].clone(), res[res.len()-1].clone()))
    }
    res
}


fn main() -> Result<()>{
    let mat  = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96,  342, 965, 150],
        vec![630, 803, 746, 422, 111],
        vec![537, 699, 497, 121, 956],
        vec![805, 732, 524, 37,  331],
    ];

    let mat = transpose(mat);

    let file = File::open("p082_matrix.txt")?;
    let mat80v : Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect();

    let mat80v = transpose(mat80v);

    println!("{:?}", mat);
    println!("------");
    let sp = spath(mat);
    println!("{:?}", sp);
    println!("{:?}", sp[sp.len()-1].iter().min().unwrap());

    //println!("--large matrix 81---");
    //println!("{:?}", mat);
    //println!("------");
    //println!("{}", spath_memo(&mat81v));


    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_map() {
        let vn = vec![18, 150, 111, 956, 331];
        let v = vec![103, 965, 422, 121, 37];

        let minpath = min_map(v, vn);

        println!("{:?}", minpath);

    }


}
