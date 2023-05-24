use std::cmp::max;
use std::cmp::min;
use std::io;
fn gcd(a: i64, b: i64) -> i64 {
    if b > a {
        return gcd(b, a);
    } else {
        if a % b == 0 {
            return b;
        } else {
            return gcd(b, a % b);
        }
    }
}
fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (a, b): (i64, i64) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    if a == b {
        println!("0")
    } else {
        let (a, b) = (max(a, b), min(a, b));
        let d = a - b;

        if d <= b {
            if b % d == 0 {
                println!("0")
            } else {
                println!("{}", d - b % d);
            }
        } else {
            if d % b == 0 {
                println!("0");
                return;
            }
            let mut i: i64 = 1;
            let mut min_lcm = a * b;
            let mut ans = 0;
            while i * i <= d {
                if d % i == 0 {
                    let poss = d / i;
                    if poss >= b {
                        let add = poss - b % poss;
                        let nxt_lcm = lcm(a + add, b + add);
                        if nxt_lcm < min_lcm {
                            min_lcm = nxt_lcm;
                            ans = add;
                        }
                    }
                    let poss = i;
                    if poss >= b {
                        let add = poss - b % poss;
                        let nxt_lcm = lcm(a + add, b + add);
                        if nxt_lcm < min_lcm {
                            min_lcm = nxt_lcm;
                            ans = add;
                        }
                    }
                }
                i += 1;
            }
            println!("{}", ans);
        }
    }
}
