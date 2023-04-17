use std::io;
use std::convert::TryInto;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t : i32 = line.trim().parse().expect(" not int");
    for _t in 0..t{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n : i32 = line.trim().parse().expect(" not int");
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
    
        let a : Vec<i32> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
    
        let b : Vec<i32> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        
        let mut s : i32 = 0;
        for i in  0..(n as usize) {
            s += a[i];
            s += b[i];
        }
        let mut f : Vec<bool> = vec![false; (s+1).try_into().unwrap()];
        f[0] = true;
        for i in 0..(n as usize) {
            for j in (0..(s+1)).rev() {
                if f[j as usize] {
                    if j + a[i] <= s {
                        f[(j+a[i]) as usize] = true;
                    }
                    if j + b[i] <= s {
                        f[(j+b[i]) as usize] = true;
                    }
                    f[j as usize] = false;
                }
            }
        }
        let mut best = -1;
        for j in 0..(s+1) {
            if f[j as usize] {
                if (2*j-s).abs() < (2*best-s).abs() {
                    best = j;
                }
            }
        }
        let mut ans = best * best + (s-best)*(s-best);
        let mut square_sum : i32 = 0;
        for i in 0..(n as usize) {
            square_sum += a[i]*a[i] + b[i]*b[i];
        }
        ans += square_sum * (n-2) ;
        println!("{}", ans);

    }
   
}
