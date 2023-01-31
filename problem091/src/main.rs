use itertools::*;

fn isright(x0:i32,y0:i32,x1:i32,y1:i32,x2:i32,y2:i32) -> bool {
    let a2 = i32::pow(x0-x1,2) + i32::pow(y0-y1,2);
    let b2 = i32::pow(x1-x2,2) + i32::pow(y1-y2,2);
    let c2 = i32::pow(x2-x0,2) + i32::pow(y2-y0,2);
    a2 == b2 + c2 || b2 == c2 + a2 || c2 == a2 + b2
}

fn iscolin(x0:i32,y0:i32,x1:i32,y1:i32,x2:i32,y2:i32) -> bool {
    x0*(y1-y2) + x1*(y2-y0) + x2*(y0-y1) == 0
}


fn main() {
    let n = 50;
    let mut count = 0;
    for (x1,y1,x2,y2) in iproduct!((0..=n),(0..=n),(0..=n),(0..=n)) {
        if iscolin(0,0,x1,y1,x2,y2) {
            continue;
        }
        
        if isright(0,0,x1,y1,x2,y2) {
            //println!("({x1},{y1}),({x2},{y2})");
            count += 1;
        }
    
        if count % 100 == 0 {
            println!("count = {}", count / 2);
        }
    }

    count /= 2;

    println!("count {n} = {count}");
}
