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
        let (n, k): (usize, i32) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let mut mat: Vec<Vec<i32>> = Vec::new();
        for _i in 0..n {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");

            let a: Vec<i32> = line
                .trim()
                .split(" ")
                .map(|x| x.parse().expect("Not an integer!"))
                .collect();
            mat.push(a);
        }

        let mut d = 0;
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != mat[n - 1 - i][n - 1 - j] {
                    d += 1;
                }
            }
        }
        d = d / 2;
        if k >= d && (n % 2 == 1 || (k - d) % 2 == 0) {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
