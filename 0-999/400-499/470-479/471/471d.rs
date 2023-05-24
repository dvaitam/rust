use std::io;
fn fail(b: &[i32], t: &mut [usize; 1000000], m: usize) {
    t[0] = usize::MAX;
    let mut pos: usize = 1;
    let mut cnd: usize = 0;
    while pos <= m {
        if cnd == 0 || b[pos - 1] == b[cnd - 1] {
            t[pos] = cnd;
            pos += 1;
            cnd += 1;
        } else {
            cnd = t[cnd];
        }
    }
}
fn matches(a: &[i32], b: &[i32], t: &mut [usize; 1000000], n: usize, m: usize) -> i32 {
    let mut res = 0;
    let mut pos: usize = 0;
    let mut cnd: usize = 0;
    while pos < n {
        if cnd == usize::MAX || (cnd < m && b[cnd] == a[pos]) {
            if cnd == usize::MAX {
                cnd = 0
            } else {
                cnd += 1;
            }
            pos += 1;
            if cnd == m {
                res += 1;
            }
        } else {
            cnd = t[cnd];
        }
    }
    return res;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (mut n, mut m): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut a: Vec<i32> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut b: Vec<i32> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    if m == 1 {
        println!("{}", n);
        return;
    }
    n -= 1;
    m -= 1;
    for i in 0..n {
        a[i] -= a[i + 1];
    }
    for i in 0..m {
        b[i] -= b[i + 1];
    }
    let mut t: [usize; 1000000] = [0; 1000000];
    fail(&b, &mut t, m);
    println!("{}", matches(&a, &b, &mut t, n, m));
}
