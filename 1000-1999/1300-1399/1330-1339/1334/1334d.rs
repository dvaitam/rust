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
        let n: i64 = split.next().unwrap().parse().expect("not int");
        let mut l: i64 = split.next().unwrap().parse().expect("not int");
        let r: i64 = split.next().unwrap().parse().expect("not int");

        if l == n * (n - 1) + 1 {
            println!("1");
        } else {
            let mut start: i64 = 1;
            let mut sm: i64 = 0;
            while sm + 2 * (n - start) < l {
                sm += 2 * (n - start);
                start += 1;
            }
            let mut second = start + (l - sm) / 2;
            if (l - sm) % 2 == 0 {
                print!("{} ", second);
                second += 1;
                l += 1;
            } else {
                second += 1;
            }
            while l <= r {
                if second == n + 1 {
                    start += 1;
                    second = start + 1;
                }
                if start >= n {
                    print!("1");
                    break;
                }
                if r - l > 0 {
                    print!("{} {} ", start, second);
                    second += 1;
                    l += 2;
                } else {
                    print!("{} ", start);
                    l += 1;
                }
                //write(f, "second here is ", second, "\n")
                if second == n + 1 {
                    start += 1;
                    second = start + 1;
                }
            }
            println!()
        }
    }
}
