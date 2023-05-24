use std::io;
#[derive(Copy, Clone, PartialEq)]
struct Symbol {
    ch: char,
    count: usize,
}
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
        let n: usize = line.trim().parse().expect("not a number");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let s: Vec<char> = line.trim().chars().collect();
        let mut symbols: Vec<Symbol> = Vec::new();
        for i in 0..n {
            if symbols.len() == 0 {
                symbols.push(Symbol { ch: s[i], count: 1 });
            } else {
                let last = symbols[symbols.len() - 1];
                if last.ch == s[i] {
                    let ll = symbols.len();
                    symbols[ll - 1].count += 1;
                //last.count++
                } else {
                    symbols.push(Symbol { ch: s[i], count: 1 });
                }
            }
        }

        let mut segments = 0;
        let mut sm = 0;
        let mut last: Option<Symbol> = None;
        for i in 0..symbols.len() {
            if sm & 1 == 1 {
                if symbols[i].count > 2 {
                    if let Some(x) = last {
                        if symbols[i].ch != x.ch {
                            segments += 1;
                        }
                        last = Some(symbols[i]);
                    } else {
                        last = Some(symbols[i]);
                        segments += 1;
                    }
                }
            } else {
                if symbols[i].count > 1 {
                    if let Some(x) = last {
                        if symbols[i].ch != x.ch {
                            segments += 1;
                        }
                        last = Some(symbols[i]);
                    } else {
                        last = Some(symbols[i]);
                        segments += 1;
                    }
                }
            }
            sm += symbols[i].count;
        }
        if segments == 0 {
            segments += 1;
        }

        let mut ans = 0;
        let mut started = usize::MAX;
        for i in 0..symbols.len() {
            if symbols[i].count & 1 == 1 {
                if started != usize::MAX {
                    ans += i - started;
                    started = usize::MAX;
                } else {
                    started = i;
                }
            }
        }
        println!("{} {}", ans, segments);
    }
}
