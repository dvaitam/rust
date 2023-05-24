use std::io;

fn main() {
    let (mut a, mut b): (i64, i64) = (1, 2);
    for _i in 0..50 {
        println!("? {} {}", a, b);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let l1: i64 = line.trim().parse().expect("not int");
        println!("? {} {}", b, a);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let l2: i64 = line.trim().parse().expect("not int");
        if l1 == -1 || l2 == -1 {
            (a, b) = (1, 2);
            continue;
        }
        if l1 != l2 {
            println!("! {}", l1 + l2);
            break;
        }
        a += 1;
        b += 1;
    }
}
