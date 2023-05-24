use std::cmp::min;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n: usize = line.trim().parse().expect("not int");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    if s.ends_with('\n') {
        s.pop();
    }
    let s: Vec<_> = s.chars().collect();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let c: Vec<i64> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let (mut h, mut a, mut r, mut d): (i64, i64, i64, i64) = (0, 0, 0, 0);
    for i in 0..n {
        if s[i] == 'h' {
            h += c[i];
        } else if s[i] == 'a' {
            a = min(h, a + c[i]);
        } else if s[i] == 'r' {
            r = min(a, r + c[i])
        } else if s[i] == 'd' {
            d = min(r, d + c[i])
        }
    }

    println!("{}", d);
}
