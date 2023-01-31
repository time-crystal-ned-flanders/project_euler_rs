// sudoku algorithm
// go by row, column, or block
// for row 1 - generate len 9 permutations of 1..9

fn col(s : Vec<u8>, i : usize) -> Vec<u8>  {
    // s[i+9*j] j=0..9
    s.into_iter().skip(i).step_by(9).collect()
}

fn row(s : Vec<u8>, i : usize) -> Vec<u8> {
    // s[9*i + j] j=0..9
    s.into_iter().skip(9*i).take(9).collect()
}

fn block(s : Vec<u8>, i : usize) -> Vec<u8> {
    let block_index = vec![0,3,6,27,30,33,54,57,60];
    let bi = block_index[i];
    let mut res = Vec::new();
    for j in 0..3 {
        for k in 0..3 {
            res.push(s[bi + 9*j + k]);
        }
    }
    res
}

fn check_sudoku_set(s : &Vec<u8>) -> bool {
    // determine if a given set contains digits 1..9 each exactly once
    let mut digit_count : Vec<u8> = vec![0; 9];
    for i in 0..9 {
        digit_count[s[i] as usize - 1] += 1;
        if digit_count[s[i] as usize - 1] > 1 {
            return false
        }
    }
    true
}

fn missing_digits(s : &Vect<u8>) -> bool { 
    
}



fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_col_row() {
        let test_sudo : Vec<u8> = "003020600900305001001806400008102900700000008006708200002609500800203009005010300".chars().map(|c| c as u8 - ('0' as u8)).collect();
        //let test_soln = "483921657967345821251876493548132976729564138136798245372689514814253769695417382".chars().map(|c| c as u8).collect();
    
        for i in 0..9 {
            println!("{:?}", col(test_sudo.clone(), i));
        }
        println!("---------\n");
        for i in 0..9 {
            println!("{:?}", row(test_sudo.clone(), i));
        }
        println!("---------\n");

        for i in 0..9 {
            println!("{:?}", block(test_sudo.clone(), i));
        }
    }

    #[test]
    fn test_set_check() {
        //let test_sudo : Vec<u8> = "003020600900305001001806400008102900700000008006708200002609500800203009005010300".chars().map(|c| c as u8 - ('0' as u8)).collect();
        let mut test_soln : Vec<u8> = "483921657967345821251876493548132976729564138136798245372689514814253769695417382".chars().map(|c| c as u8 - ('0' as u8)).collect();
    
        test_soln[34] = 4;

        for i in 0..9 {
            let s = col(test_soln.clone(), i);
            println!("{:?} - {}", s, check_sudoku_set(&s));
        }
        println!("---------\n");
        for i in 0..9 {
            let s = row(test_soln.clone(), i);
            println!("{:?} - {}", s, check_sudoku_set(&s));
        }
        println!("---------\n");
        for i in 0..9 {
            let s = block(test_soln.clone(), i);
            println!("{:?} - {}", s, check_sudoku_set(&s));
        }
        println!("---------\n");
        
    }


}