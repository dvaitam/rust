use std::io;
fn gcd(a: i32, b: i32) -> i32 {
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
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not int");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (a, b): (i32, i32) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        if gcd(a, b) == 1 {
            println!("{}", 1);
            println!("{} {}", a, b);
        } else {
            println!("{}", 2);
            if gcd(a + 1, b) == 1 && a != 1000000000 {
                println!("{} {}", a + 1, b);
                println!("{} {}", a, b);
            } else if gcd(a, b + 1) == 1 && b != 1000000000 {
                println!("{} {}", a, b + 1);
                println!("{} {}", a, b);
            } else if gcd(a - 1, b) == 1 {
                println!("{} {}", a - 1, b);
                println!("{} {}", a, b);
            } else if gcd(a, b - 1) == 1 {
                println!("{} {}", a, b - 1);
                println!("{} {}", a, b);
            } else if gcd(a - 1, b - 1) == 1 {
                println!("{} {}", a - 1, b - 1);
                println!("{} {}", a, b);
            } else {
                println!("{} {}", a + 1, b + 1);
                println!("{} {}", a, b);
            }
        }
    }
}
