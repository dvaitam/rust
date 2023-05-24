use std::io;

fn main() {
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
    let (mut m, mut n) = (true, true);
    let (mut same, mut diff) = (true, true);
    for i in 0..t.len() {
        if s[i] == t[i] {
            m = !m;
        } else {
            n = !n;
        }
        if i > 0 {
            if t[i] == t[i - 1] {
                same = !same;
            } else {
                diff = !diff;
            }
        }
    }
    let mut ans = 0;
    if n {
        ans += 1;
    }
    //write(f, "ans ", ans, "\n")
    for i in t.len()..s.len() {
        if s[i - t.len()] == t[0] {
            m = !m;
        } else {
            n = !n;
        }
        if !diff {
            m = !m;
            n = !n;
        }
        if s[i] == t[t.len() - 1] {
            m = !m;
        } else {
            n = !n;
        }

        if n {
            ans += 1;
        }
    }
    println!("{}", ans);
}
