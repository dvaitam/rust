use std::cmp::min;
use std::io;
fn float64(a: usize) -> f64 {
    a as f64
}
fn get_prob(w: usize, b: usize) -> f64 {
    if w == 0 {
        return 0.0;
    }
    if b == 0 {
        return 1.0;
    }

    let mut ans = float64(w) / float64(w + b);
    if b == 1 {
        return ans;
    }
    if b > 1 {
        ans += (float64(b * (b - 1) * w) * get_prob(w - 1, b - 2))
            / float64((w + b) * (w + b - 1) * (w + b - 2));
    }
    if b > 2 {
        ans += (float64(b * (b - 1) * (b - 2)) * get_prob(w, b - 3))
            / float64((w + b) * (w + b - 1) * (w + b - 2));
    }
    return ans;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (w, b): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; w + 1]; b + 1];

    for bi in 0..min(3, b + 1) {
        for wi in 0..(w + 1) {
            dp[bi][wi] = get_prob(wi, bi);
        }
    }
    for wi in 0..min(2, w + 1) {
        for bi in 0..b + 1 {
            dp[bi][wi] = get_prob(wi, bi);
        }
    }
    for bi in 3..b + 1 {
        for wi in 2..w + 1 {
            dp[bi][wi] = float64(wi) / float64(wi + bi);
            dp[bi][wi] += (float64(bi * (bi - 1) * (bi - 2)) * dp[(bi - 3)][wi]
                + float64(bi * (bi - 1) * wi) * dp[(bi - 2)][(wi - 1)])
                / float64((wi + bi) * (wi + bi - 1) * (wi + bi - 2));
        }
    }
    println!("{}", dp[b][w]);
}
