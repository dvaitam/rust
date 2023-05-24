use std::io;
#[derive(Copy, Clone)]
struct Char {
    c: char,
    i: usize,
}

fn other(s: &str) -> &str {
    if s == "1" {
        return "0";
    } else {
        return "1";
    }
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n: usize = line.trim().parse().expect(" not int");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    if s.ends_with('\n') {
        s.pop();
    }
    if s.ends_with('\r') {
        s.pop();
    }
    if s.ends_with('\n') {
        s.pop();
    }
    if s.ends_with('\r') {
        s.pop();
    }
    let s: Vec<char> = s.chars().collect();
    let mut chars: Vec<Char> = vec![Char { c: 'c', i: 0 }; n];
    for i in 0..n {
        chars[i] = Char { c: s[i], i: i };
    }
    let mut ans: Vec<&str> = vec!["-1"; n];
    let mut ok: bool = true;
    for i in 0..n {
        if i > 0 && chars[i].c < chars[i - 1].c {
            let mut j = i;
            while j > 0 && chars[j].c < chars[j - 1].c {
                if ans[chars[j - 1].i] == "-1" {
                    ans[chars[j - 1].i] = "1";
                }
                let oth = other(ans[chars[j - 1].i]);
                if ans[chars[j].i] == "-1" {
                    ans[chars[j].i] = oth;
                } else {
                    if oth != ans[chars[j].i] {
                        ok = false;
                    }
                }
                (chars[j], chars[j - 1]) = (chars[j - 1], chars[j]);
                j -= 1;
            }
        }
    }
    if ok {
        println!("YES");
        for i in 0..n {
            if ans[i] == "-1" {
                print!("1")
            } else {
                print!("{}", ans[i]);
            }
        }
        println!();
    } else {
        println!("NO");
    }
}
