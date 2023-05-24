use std::collections::VecDeque;
use std::io;
#[derive(Copy, Clone)]
struct Node {
    i: usize,
    j: usize,
    l: usize,
    r: usize,
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, m): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (r, c): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (x, y): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut s: Vec<Vec<char>> = vec![Vec::new(); n];
    for i in 0..n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        s[i] = line.trim().chars().collect();
    }
    let mut visited = vec![vec![false; m]; n];
    visited[r - 1][c - 1] = true;
    let mut ans = 1;
    let mut l: VecDeque<Node> = VecDeque::new();
    l.push_back(Node {
        i: r - 1,
        j: c - 1,
        l: 0,
        r: 0,
    });
    let mut sl: VecDeque<Node> = VecDeque::new();
    let mut front;
    while l.len() > 0 || sl.len() > 0 {
        if l.len() > 0 {
            front = l.pop_front().unwrap();
        } else {
            front = sl.pop_front().unwrap();
        }
        if front.i > 0 && !visited[front.i - 1][front.j] && s[front.i - 1][front.j] != '*' {
            ans += 1;
            visited[front.i - 1][front.j] = true;
            l.push_back(Node {
                i: front.i - 1,
                j: front.j,
                l: front.l,
                r: front.r,
            });
        }
        if front.i + 1 < n && !visited[front.i + 1][front.j] && s[front.i + 1][front.j] != '*' {
            ans += 1;
            visited[front.i + 1][front.j] = true;
            l.push_back(Node {
                i: front.i + 1,
                j: front.j,
                l: front.l,
                r: front.r,
            });
        }
        if front.j > 0
            && !visited[front.i][front.j - 1]
            && s[front.i][front.j - 1] != '*'
            && front.l < x
        {
            ans += 1;
            visited[front.i][front.j - 1] = true;
            sl.push_back(Node {
                i: front.i,
                j: front.j - 1,
                l: front.l + 1,
                r: front.r,
            });
        }
        if front.j + 1 < m
            && !visited[front.i][front.j + 1]
            && s[front.i][front.j + 1] != '*'
            && front.r < y
        {
            ans += 1;
            visited[front.i][front.j + 1] = true;
            sl.push_back(Node {
                i: front.i,
                j: front.j + 1,
                l: front.l,
                r: front.r + 1,
            });
        }
    }
    println!("{}", ans);
}
