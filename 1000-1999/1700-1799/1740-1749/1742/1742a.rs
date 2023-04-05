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
        let mut split = line.trim().split(" ");
        let (a, b, c) = (
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        );
        let (a, b, c): (i32, i32, i32) = (
            a.parse().expect("not int"),
            b.parse().expect("not int"),
            c.parse().expect("not int"),
        );
        if a + b - c == 0 || b + c - a == 0 || a + c - b == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
