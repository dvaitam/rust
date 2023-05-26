use std::collections::HashMap;
use std::io;
fn pow(x: i64, n: usize) -> i64 {
    if n == 0 {
        1
    } else {
        x * pow(x, n - 1)
    }
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, k): (usize, i64) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let a: Vec<i64> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut c: Vec<i64> = vec![0; n];
    for i in 0..n {
        if i == 0 {
            c[i] = a[i];
        } else {
            c[i] = c[i - 1] + a[i];
        }
    }
    let mut m: HashMap<i64, i64> = HashMap::new();
    m.entry(0).or_insert(1);
    let mx = pow(10, 15);
    let mut ans: i64 = 0;
    for i in 0..n {
        for j in 0..60 {
            let val = pow(k, j);
            if k == 1 && j == 1 || k == -1 && j == 2 || val.abs() > mx {
                break;
            }
            if m.contains_key(&(c[i] - val)) {
                ans += m[&(c[i] - val)];
            }
        }
        let count = m.entry(c[i]).or_insert(0);
        *count += 1;
    }
    println!("{}", ans);
}
