use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: u32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n: usize = line.trim().parse().expect("not a int");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let s: Vec<char> = line.trim().to_string().chars().collect();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let t: Vec<char> = line.trim().to_string().chars().collect();
        if s[0] != t[0] || s[n - 1] != t[n - 1] {
            println!("-1");
            continue;
        }
        let mut ss: Vec<i64> = Vec::new();
        let mut tt: Vec<i64> = Vec::new();
        for i in 0..n {
            if i == 0 {
                ss.push(1);
                tt.push(1);
            } else {
                if s[i] == s[i - 1] {
                    let ll = ss.len();
                    ss[ll - 1] += 1;
                } else {
                    ss.push(1);
                }
                if t[i] == t[i - 1] {
                    let ll = tt.len();
                    tt[ll - 1] += 1;
                } else {
                    tt.push(1);
                }
            }
        }
        if ss.len() != tt.len() {
            println!("-1");
            continue;
        }
        let mut diff: Vec<i64> = vec![0; ss.len()];
        for i in 0..ss.len() {
            diff[i] = ss[i] - tt[i];
        }
        let mut ans = 0;
        for i in 0..diff.len() {
            if diff[i] != 0 {
                diff[i + 1] += diff[i];
                ans += diff[i].abs();
                diff[i] = 0;
            }
        }
        println!("{}", ans);
    }
}
