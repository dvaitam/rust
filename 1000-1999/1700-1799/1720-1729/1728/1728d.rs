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
        let s: Vec<char> = line.trim().chars().collect();
        let mut start: Vec<bool> = vec![false; s.len()];
        let mut front: Vec<bool> = vec![false; s.len()];
        let mut back: Vec<bool> = vec![false; s.len()];

        for i in 0..(s.len() - 1) {
            if s[i] == s[i + 1] {
                front[i] = true;
            }
        }
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                back[i] = true;
            }
        }
        let mut l = 2;
        while start.len() > 1 {
            if l % 2 == 0 {
                start = vec![false; s.len() - l + 1];
                for i in 0..(s.len() - l + 1) {
                    start[i] = front[i] && back[i + 1];
                }
            } else {
                front = vec![false; s.len()];
                back = vec![false; s.len()];
                for i in 0..(s.len() - l) {
                    if s[i] == s[i + l] && s[i + l] == s[i + l - 1] {
                        front[i] = start[i] || start[i + 1];
                    } else if s[i] == s[i + l] {
                        front[i] = start[i + 1];
                    } else if s[i + l] == s[i + l - 1] {
                        front[i] = start[i];
                    } else {
                        front[i] = false;
                    }
                }
                for i in 1..(s.len() - l + 1) {
                    if s[i] == s[i - 1] && s[i - 1] == s[i + l - 1] {
                        back[i] = start[i + 1] || start[i];
                    } else if s[i] == s[i - 1] {
                        back[i] = start[i + 1];
                    } else if s[i - 1] == s[i + l - 1] {
                        back[i] = start[i];
                    } else {
                        back[i] = false;
                    }
                }
            }
            l += 1;
        }

        if start[0] {
            println!("Draw");
        } else {
            println!("Alice");
        }
    }
}
