use std::io;
use std::cmp::min;
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
        let (_n, mut c) : (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        c +=1;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let b : Vec<usize> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
        let mut a : Vec<usize> = vec![0;c];
        for i in 0..b.len() {
            a[b[i]] = 1
        }
        let mut sum : Vec<usize> = vec![0;c+1];
        for i in 1..c+1 {
            sum[i] = sum[i-1] + a[i-1];
        }
        let mut failed = false;
        for i in 1..c {
            if a[i] == 0 {
                continue;
            }
            for j in 1..c {
                if j*i >= c {
                    break;
                }
                if a[j] != 0 {
                    continue;
                }
                let x : usize = i * j;
                let y : usize= min(i * (j+1), c);
                if sum[y] - sum[x] > 0 {
                    println!("NO");
                    failed = true;
                    break;
                }

            }
            if failed {
                break;
            }
        }
        if !failed {
            println!("YES");
        }


    }
}
