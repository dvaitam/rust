use std::io;
use std::cmp::min;
use std::cmp::max;
fn get_wins(a1 : i32, a2 :i32, a3 :i32, b1 :i32, b2 :i32, b3: i32) -> i32 {
	 min(b1, a2) + min(b2, a3) + min(b3, a1)
}

fn get_losses(a1 : i32, a2 :i32, a3 :i32, b1 :i32, b2 :i32, b3: i32) -> i32{
	max(b1-min(b1, a1+a2), max(b2-min(b2, a2+a3), b3-min(b3, a1+a3)))
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let _n : i32= line.trim().parse().expect("not a number");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (a1, a2, a3) = (
        split.next().unwrap(),
        split.next().unwrap(),
        split.next().unwrap(),
    );
    let (a1, a2, a3): (i32, i32, i32) = (
        a1.parse().expect("not int"),
        a2.parse().expect("not int"),
        a3.parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (b1, b2, b3) = (
        split.next().unwrap(),
        split.next().unwrap(),
        split.next().unwrap(),
    );
    let (b1, b2, b3): (i32, i32, i32) = (
        b1.parse().expect("not int"),
        b2.parse().expect("not int"),
        b3.parse().expect("not int"),
    );

    println!("{} {}",get_losses(a1, a2, a3, b1, b2, b3), get_wins(b1, b2, b3, a1, a2, a3));
}
