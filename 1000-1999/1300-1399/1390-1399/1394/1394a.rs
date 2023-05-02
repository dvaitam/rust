use std::io;
use std::cmp::max;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, d, m): (usize, usize, i64) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut a : Vec<i64> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();  
    a.sort();
    let mut l : Vec<i64> = Vec::new();
    let mut h : Vec<i64> = Vec::new();
    for i in 0..n {
        if a[i] <= m {
            l.push(a[i]);
        }else{
            h.push(a[i]);
        }
    }
    let mut ldp : Vec<i64> = vec![0;l.len()];
    let mut hdp : Vec<i64> = vec![0;h.len()];
    for i in 0..l.len() {
        if i == 0 {
            ldp[i] = l[i];
        }else{
            ldp[i] = ldp[i-1] + l[i];
        }
    }
    for i in (0..h.len()).rev() {
        if i == h.len()-1 {
            hdp[i] = h[i];
        }else{
            hdp[i] = hdp[i+1] + h[i];
        }
    }
    let mut dp : Vec<i64> = vec![0;l.len()+1];
    for i in 0..(l.len()+1){
		let mut tmp : i64 = 0;
		if l.len() > 0 {
			tmp += ldp[l.len()-1];
		}
		if i > 0 {
			tmp -= ldp[i-1];
		}
		let rem = i + h.len();
		let mut total = rem / (d + 1);
		if rem%(d+1) != 0 {
			total += 1;
		}
		if hdp.len() >= total  && hdp.len()-total < hdp.len() {
			tmp += hdp[hdp.len()-total];
		}
		dp[i] = tmp;
	}
    let mut ans = dp[0];
    for i in 1..(l.len()+1) {
        ans = max(ans, dp[i]);
    }
    println!("{}", ans);
}