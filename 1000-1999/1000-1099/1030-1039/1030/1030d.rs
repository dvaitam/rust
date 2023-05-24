use std::io;
fn gcd(a: usize, b: usize) -> usize {
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
    let mut split = line.trim().split(" ");
    let (mut n, mut m, mut k): (usize, usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    if k % 2 == 0 {
        k = k / 2;
        if k == 1 {
            println!("YES");
            println!("{} {}", 0, 0);
            println!("{} {}", 0, m);
            println!("{} {}", n, 0);
        } else {
            if (n * m) % k != 0 {
                println!("NO");
            } else {
                let gd = gcd(n, k);
                n = n / gd;
                k = k / gd;
                m = m / k;
                println!("YES");
                println!("{} {}", 0, 0);
                println!("{} {}", 0, m);
                println!("{} {}", n, 0);
            }
        }
    } else {
        if (n * m) % k != 0 {
            println!("NO");
        } else {
            let mut gd = gcd(n, k);
            if gd > 2 {
                n = (n / gd) * 2;
                k = k / gd;
                m = m / k;
            } else {
                gd = gcd(m, k);
                m = (m / gd) * 2;
            }
            println!("YES");
            println!("{} {}", 0, 0);
            println!("{} {}", 0, m);
            println!("{} {}", n, 0);
        }
    }
}
