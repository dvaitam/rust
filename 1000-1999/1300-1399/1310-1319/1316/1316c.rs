use std::io;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, m, p) = (
        split.next().unwrap(),
        split.next().unwrap(),
        split.next().unwrap(),
    );
    let (_n, _m, p): (i32, i32, i32) = (
        n.parse().expect("not int"),
        m.parse().expect("not int"),
        p.parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let a : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let b : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let  (mut ai, mut bi) : (i32, i32) = (-1,-1);
    for i in 0..a.len() {
        if a[i]%p != 0 && ai == -1 {
            ai = i as i32;
        }
    }
    for i in 0..b.len() {
        if b[i]%p != 0 && bi == -1 {
            bi = i as i32;
        }
    }
    println!("{}", ai+bi);
}
