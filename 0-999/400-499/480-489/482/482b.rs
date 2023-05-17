use std::io;
fn main() {
    
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, m) : (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut from : Vec<usize> = vec![0;m];
    let mut to : Vec<usize> = vec![0;m];
    let mut num : Vec<usize> = vec![0;m];
    let mut s : Vec<Vec<i32>> = vec![vec![0;30];n+1];
    let mut sum : Vec<Vec<i32>> = vec![vec![0;30];n+1];
    let mut a : Vec<Vec<bool>> = vec![vec![false;30];n+1];
    for i in 0..m {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (l, r, q) : (usize, usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        from[i] = l-1;
        to[i] = r-1;
        num[i] = q;
        for j in 0..30 {
            if q & (1<<j) != 0 {
                s[l-1][j] += 1;
                s[r][j] -= 1;
            }
        }
    }
    for j in 0..30 {
        let mut base = 0;
        for i in 0..n {
            base += s[i][j];
            a[i][j] = base > 0;
            if a[i][j] {
                sum[i+1][j] = sum[i][j] + 1;
            }else{
                sum[i+1][j] = sum[i][j];
            }
        }
    }
    for i in 0..m {
        
        for j in 0..30 {
            if num[i] & (1<<j) == 0 {
                let have = sum[to[i]+1][j] - sum[from[i]][j];
                if have as usize == to[i] + 1 - from[i] {
                    println!("NO");
                    return;
                }
            }
        }
    }
    println!("YES");
    for i in 0..n {
        let mut res = 0;
        for j in 0..30 {
            if a[i][j] {
                res += 1<<j;
            }
        }
        print!("{} ", res);
    }
    println!();
}
