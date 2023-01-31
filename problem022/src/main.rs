use std::fs;
use std::fs::{File};
use std::io::{self, BufRead, Result};

fn score_word(w : String) -> u64 {
    w.as_bytes().iter().map(|x| *x as u64).map(|x| x - ('A' as u64) + 1).sum()
}


fn main() -> io::Result<()> {
    let filename = "p022_names.txt";
    let ss = fs::read_to_string(filename)?;
    let mut words : Vec<_> = ss.split(",").map(|s| s.trim_matches('\"').to_string()).collect();
    words.sort();
    
    let mut sum = 0;
    for (i,w) in words.into_iter().enumerate() {
        let ws = score_word(w.clone());
        let j = i as u64 + 1;
        println!("{}, {} {} -- {}", j, w, ws, j*ws);
        sum += j*ws;
    }
    println!("SUM = {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {




    use super::*;



    /*
    #[test]
    fn test_chars() {
        let s = "\"BENTON\"";
        println!("{}", "\"BENTON\"".trim_matches('\"'));
    }
    */

    #[test]
    fn test_score() {
        let nm = "COLIN".to_string();
        println!("{} = {}", nm, score_word(nm.clone()))
    }
}