use std::cmp::max;
use std::cmp::min;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, m, p): (usize, usize, i64) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut a: Vec<i64> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut b: Vec<i64> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    a.sort();
    b.sort();
    let mut ans: i64 = 2000000000;
    for delta in 0..(m - n + 1) {
        let mut curr: i64 = 0;
        for i in 0..n {
            curr = max(curr, (a[i] - b[i + delta]).abs() + (b[i + delta] - p).abs());
        }
        ans = min(ans, curr);
    }
    println!("{}", ans);
}
