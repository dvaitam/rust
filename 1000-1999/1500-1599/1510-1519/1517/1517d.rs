use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, m, k): (usize, usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    

    let mut h : Vec<Vec<i32>> = vec![Vec::new() ; n];
    

    let mut v : Vec<Vec<i32>> = vec![Vec::new(); n-1];

    for i in 0..n{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        h[i] = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    }
    for i in 0..n-1{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        v[i] = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    }
    if k % 2 == 1 {
        for _i in 0..n {
            for _j in 0..m {
                print!("-1 ")
            }
            println!()
        }
        return;
    }
    let mut c : Vec<Vec<Vec<i32>>> = vec![vec![vec![0;k/2]; m] ; n];

    for p in 0..k/2 {
        for i in 0..n {
            for j in 0..m {
                if p == 0 {
                    let mut poss : Vec<i32> = Vec::new();
                    if i > 0 {
                        poss.push(v[i-1][j]);
                    }
                    if i < n-1 {
                        poss.push(v[i][j]);
                    }
                    if j > 0 {
                        poss.push(h[i][j-1]);
                    }
                    if j < m-1 {
                        poss.push(h[i][j]);
                    }
                    c[i][j][p] = *poss.iter().min().unwrap();
                }else{
                    let mut poss : Vec<i32> = Vec::new();
                    if i > 0 {
                        poss.push(c[i-1][j][p-1]+v[i-1][j]);
                    }
                    if i < n-1 {
                        poss.push(c[i+1][j][p-1] + v[i][j]);
                    }
                    if j > 0 {
                        poss.push(c[i][j-1][p-1] + h[i][j-1]);
                    }
                    if j < m-1 {
                        poss.push(c[i][j+1][p-1] + h[i][j]);
                    }
                    c[i][j][p] = *poss.iter().min().unwrap();
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{} ", 2*c[i][j][(k/2)-1])
        }
        println!()
    }



    
}
