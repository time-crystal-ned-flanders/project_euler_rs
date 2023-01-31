use std::collections::*;

fn main() {
    // generate pythagorean triples
    let max = 30;
    
    let mut pcount : HashMap<u64,u64> = HashMap::new();
    
    
    for m in 1..=max {
        for n in 1..m {
            if m % 2 == 0 && n % 2 == 0 {
                continue;
            }
            let a = m*m - n*n;
            let b = 2*m*n;
            let c = m*m + n*n;
            let p = a + b + c;
            //println!("({a},{b},{c}), p = {p}");
            for k in 1..100 {
                let pk = k*p;
                *pcount.entry(pk).or_insert(0) += 1;
            }
            
        }
    }

    let maxp = pcount.iter().max_by(|a,b| a.1.cmp(&b.1)).map(|(k,_v)| k).unwrap();

    println!("max p = {maxp} {:?}", pcount[maxp]);

    dbg!(pcount[&120]);
}
