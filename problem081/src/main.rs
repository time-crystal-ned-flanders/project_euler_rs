use std::fs::{File};
use std::io::{self, BufRead, Result};
use std::convert::TryInto;

fn spath(i: usize, j: usize, mat : &Vec<Vec<u32>>) -> u32 {
    if i == mat.len()-1 && j == mat[0].len()-1 {
        mat[i][j]
    } else if i == mat.len()-1 && j < mat[0].len()-1 {
        mat[i][j] + spath(i,j+1,mat)
    } else if i < mat.len()-1 && j == mat[0].len()-1 {
        mat[i][j] + spath(i+1,j,mat)
    } else {
        mat[i][j] + u32::min(spath(i+1,j,mat), spath(i,j+1,mat))
    }
}

fn spath_memo(mat : &Vec<Vec<u32>>) -> u32 {
    let (h,w) = (mat.len(), mat[0].len());
    println!("{:?}",(h,w));
    //let mut minmat : Vec<Vec<_>> = vec![vec![0; w+1]; h+1];
    let mut minmat = mat.clone();
    
    for i in 0..w {
        for j in 0..h {
            if i == 0 && j == 0 {
                minmat[i][j] += 0;
            } else if i == 0 && j > 0 {
                minmat[i][j] += minmat[i][j-1];
            } else if i > 0 && j == 0 {
                minmat[i][j] += minmat[i-1][j];
            } else {
                minmat[i][j] += u32::min(minmat[i-1][j] , minmat[i][j-1]);
            }
        }
    }
    minmat[h-1][w-1]
}

fn main() -> Result<()> {
    let mat  = vec![
        vec![131, 673, 234, 103, 18],
        vec![201, 96,  342, 965, 150],
        vec![630, 803, 746, 422, 111],
        vec![537, 699, 497, 121, 956],
        vec![805, 732, 524, 37,  331],
    ];

    let file = File::open("p081_matrix.txt")?;
    let mat81v : Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect();

    println!("{:?}", mat);
    println!("------");
    println!("{}", spath_memo(&mat));

    println!("--large matrix 81---");
    //println!("{:?}", mat);
    println!("------");
    println!("{}", spath_memo(&mat81v));


    Ok(())
}



