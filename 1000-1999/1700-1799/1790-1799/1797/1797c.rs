use std::io;
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
        let (n, _m): (i32, i32) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        println!("? {} {}", 1, 1);
        // io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let m1: i32 = line.trim().parse().expect("not int");

        println!("? {} {}", n, 1);
        // io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let m2: i32 = line.trim().parse().expect("not int");

        if m1 + m2 > n - 1 {
            if m1 < m2 {
                println!("! {} {}", n - m2, m1 + 1);
            } else if m2 < m1 {
                println!("! {} {}", m1 + 1, m2 + 1);
            } else {
                println!("? {} {}", 1, m1 + 1);

                let mut line = String::new();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let m3: i32 = line.trim().parse().expect("not int");
                println!("! {} {}", m3 + 1, m1 + 1);
            }
        } else {
            println!("? {} {}", m1 + 1, 1);
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let m3: i32 = line.trim().parse().expect("not int");
            println!("! {} {}", m1 + 1, m3 + 1)
        }
    }
}
