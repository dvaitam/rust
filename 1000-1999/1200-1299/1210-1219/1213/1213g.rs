use std::io;
#[derive(Copy, Clone)]
struct Edge {
    u: usize,
    v: usize,
    w: usize,
}
#[derive(Copy, Clone)]
struct Pair {
    i: usize,
    q: usize,
}
struct Dsu {
    p: Vec<usize>,
    sz: Vec<usize>,
    components: usize,
}
impl Dsu {
    fn get(&mut self, x: usize) -> usize {
        if x == self.p[x] {
            return x;
        } else {
            self.p[x] = self.get(self.p[x]);
            return self.p[x];
        }
    }
    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.get(x);
        let mut y = self.get(y);
        if x != y {
            if self.sz[x] < self.sz[y] {
                (x, y) = (y, x);
            }
            self.sz[x] += self.sz[y];
            self.p[y] = x;
            self.components -= 1;
            return true;
        }
        return false;
    }
    fn size(&mut self, x: usize) -> usize {
        let y = self.get(x);
        return self.sz[y];
    }
    fn new(n: usize) -> Dsu {
        let mut p: Vec<usize> = vec![0; n];
        for i in 0..n {
            p[i] = i;
        }
        let sz: Vec<usize> = vec![1; n];
        return Dsu {
            p: p,
            sz: sz,
            components: n,
        };
    }
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
    let mut e: Vec<Edge> = vec![Edge { u: 0, v: 0, w: 0 }; n - 1];
    for i in 0..n - 1 {
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
        e[i] = Edge {
            u: u - 1,
            v: v - 1,
            w: w,
        };
    }
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let q: Vec<usize> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut qs: Vec<Pair> = vec![Pair { i: 0, q: 0 }; m];
    for i in 0..m {
        qs[i] = Pair { i: i, q: q[i] };
    }
    let mut dsu = Dsu::new(n);
    e.sort_unstable_by_key(|item| (item.w));
    qs.sort_unstable_by_key(|item| (item.q));
    let mut ans: Vec<usize> = vec![0; m];
    let mut curr: usize = 0;
    let mut i = 0;
    for j in 0..m {
        while i < n - 1 && e[i].w <= qs[j].q {
            curr += dsu.size(e[i].u) * dsu.size(e[i].v);
            dsu.unite(e[i].u, e[i].v);
            i += 1;
        }
        ans[qs[j].i] = curr;
    }
    for i in 0..m {
        print!("{} ", ans[i]);
    }
    println!();
}
