use std::io;
fn check(u : i32, v : i32) -> bool {
    if u == v  {
        return true;
    }
    if u > v {
        return false;
    }
    let mut i = 30;
    while i >= 0 {
        let curr = 1<<i;
        if curr|v == curr+v && v-curr >= u {
			return check(u, v-curr);
		}
        i -= 1;
    }
    return false;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t : i32 = line.trim().parse().expect(" not int");
    for _t in 0..t{
        let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (u, v): (i32, i32) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    if check(u, v) {
        println!("YES");
    }else{
        println!("NO");
    }
}
   
}
