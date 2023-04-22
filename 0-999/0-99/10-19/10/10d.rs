use std::io;
use std::cmp::max;
fn print_ans(x : usize, step : &Vec<usize>, b : &Vec<i32>) {
    if x != 0 {
        print_ans(step[x], step, b);
        print!("{} ", b[x-1]);
    }
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n : usize = line.trim().parse().expect("not int");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let a : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let m : usize = line.trim().parse().expect("not int");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let b : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let maxn = max(n, m);
    let mut dp : Vec<usize> = vec![0;maxn+1];
    let mut step : Vec<usize> = vec![0;maxn+1];
    for i in 1..n+1{
        let mut pos : usize = 0;
        for j in 1..m+1 {
            if a[i-1] == b[j-1] {
                dp[j] = dp[pos] + 1;
                step[j] = pos;
            }else if a[i-1] > b[j-1] && dp[pos] < dp[j] {
                pos = j;
            }
        }
    }
    let mut ans : usize = 0;
    let mut ansl : usize = 0;
    for i in 1..m+1 {
        if dp[i] > ansl {
            (ansl, ans) = (dp[i], i);
        }
    }
    println!("{}", ansl);
    print_ans(ans, &step, &b);
}
