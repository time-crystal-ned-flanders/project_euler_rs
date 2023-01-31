// project euler problem 85 counting rectangles

use std::num;

fn smn(m: i64, n: i64, c: i64, r: i64) -> i64 {
    (c - m + 1)*(r - n + 1)
}

fn tmn(c: i64, r: i64) -> i64 {
    let mut res = 0;
    for m in 1..c+1 {
        for n in 1..r+1 {
            let sp = smn(m,n,c,r);
            println!("smn({m},{n},{c},{r}) = {sp}");
            res += sp;
        }
    }
    res
}


fn tmnf(c: i64, r: i64) -> i64 {
    let ct = c*r*(c*r+c+r+1);
    let mut mnt = 0;
    for m in 1..c+1 {
        for n in 1..r+1 {
            mnt += m*n - m*(r+1) - n*(c+1);
        }
    }
    ct + mnt
}

// correct sum of series

fn tmnff(c: i64, r: i64) -> i64 {
    //c*r*(c*r+c+r+1) + c*r*(c+1)*(r+1)/4 - c*(c+1)*r*(r+1)/2 - r*(r+1)*c*(c+1)/2;
    c*(c+1)*r*(r+1)/4
}


fn main() {
    let t32 = tmn(3,2);
    let t32f = tmnf(3,2);
    let t32ff = tmnff(3,2);

    println!("t32 = {t32}");
    println!("t32f = {t32f}");
    println!("t32ff = {t32ff}");

    let goal = 2000000;
    let mut c = 1;
    let mut best = 2000000;
    loop {
        for r in 1..c+1 {
            let tcr = tmnff(c,r);
            let diff = (goal - tcr).abs();
            if diff < best {
                best = diff;
                println!("new best: {c},{r} = {best}");
            }
        }
        c += 1;
    }


}


#[cfg(test)]
mod test {

}

