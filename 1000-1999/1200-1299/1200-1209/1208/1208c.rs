use std::io;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n: usize = line.trim().parse().expect("not int");
    let mut a: Vec<Vec<i32>> = vec![vec![0; n]; n];
    let mut row = true;
    let mut curr = 0;
    let mut i = 0;
    while i < n {
        let mut coll = row;
        let mut j = 0;
        while j < n {
            a[i][j] = curr;
            a[i][j + 1] = curr + 1;
            a[i + 1][j] = curr + 2;
            a[i + 1][j + 1] = curr + 3;
            if !coll {
                a[i][j + 1] = curr + 2;
                a[i + 1][j] = curr + 1;
            }
            curr += 4;
            coll = !coll;
            j += 2;
        }
        row = !row;
        i += 2;
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", a[i][j]);
        }
        println!();
    }
}
