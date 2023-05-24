use std::io;
const MD: i64 = 1000000007;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect(" not int");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n: usize = line.trim().parse().expect(" not int");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let x: Vec<i64> = line
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut sm: Vec<i64> = vec![0; 64];
        let mut j = 0;
        while j < 63 {
            let curr: i64 = 1 << j;
            for i in 0..n {
                if x[i] & curr == curr {
                    sm[j] += curr % MD;
                    sm[j] %= MD;
                }
            }
            j += 1;
        }
        let mut ans: i64 = 0;

        for i in 0..n {
            let mut or_sum: i64 = 0;
            let mut and_sum: i64 = 0;

            let mut j = 0;
            while j < 63 {
                let curr: i64 = 1 << j;
                if x[i] & curr == curr {
                    or_sum += (curr % MD) * (n as i64);
                    or_sum %= MD;
                    and_sum += sm[j];
                    and_sum %= MD;
                } else {
                    or_sum += sm[j];
                    or_sum %= MD;
                }
                j += 1;
            }
            ans += (or_sum * and_sum) % MD;
            ans %= MD;
        }
        println!("{}", ans);
    }
}
