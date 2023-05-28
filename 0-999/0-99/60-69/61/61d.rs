use std::cmp::max;
use std::io;
#[derive(Copy, Clone)]
struct Edge {
    v: usize,
    w: usize,
}
fn longest(x: usize, parent: usize, adj: &Vec<Vec<Edge>>) -> usize {
    let mut lng = 0;
    for k in adj[x].iter() {
        if k.v == parent {
            continue;
        }
        lng = max(lng, k.w + longest(k.v, x, adj));
    }
    return lng;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n: usize = line.trim().parse().expect("not int");
    let mut adj: Vec<Vec<Edge>> = vec![Vec::new(); n + 1];
    let mut ans = 0;
    for _i in 0..n - 1 {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");

        let (u, v, w): (usize, usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        adj[u].push(Edge { v: v, w: w });
        adj[v].push(Edge { v: u, w: w });
        ans += 2 * w;
    }
    ans -= longest(1, 0, &adj);

    println!("{}", ans);
}
