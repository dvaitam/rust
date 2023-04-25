use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n: i32 = line.trim().parse().expect("not a number");
        if n == 1 {
            println!("1")
        }else if n % 2 == 1 {
            println!("-1")
        }else{
            let mut neg = true;
            if n % 4 == 0 {
                neg = false;
            }
            print!("{}",n);
            for i in 1..n/2 {

                if neg {
                    print!(" {}", n-i);
                } else {
                    print!(" {}", i);
                }
                neg = !neg;
            }
            print!(" {}", n/2);
            for i in (1..n/2).rev() {
                if neg {
                    print!(" {}", n-i);
                } else {
                    print!(" {}", i);
                }
                neg = !neg;
            }
            println!()
        }
    }
}
