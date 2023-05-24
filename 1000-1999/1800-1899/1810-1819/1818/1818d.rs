use std::collections::HashSet;
use std::io;
#[derive(Copy, Clone)]
struct Edge {
    u: usize,
    v: usize,
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");

        let (n, m): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut degree: Vec<usize> = vec![0; n + 1];
        let mut edges: Vec<Edge> = Vec::new();
        for _i in 0..m {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let mut split = line.trim().split(" ");

            let (u, v): (usize, usize) = (
                split.next().unwrap().parse().expect("not int"),
                split.next().unwrap().parse().expect("not int"),
            );
            degree[u] += 1;
            degree[v] += 1;
            adj[u].push(v);
            adj[v].push(u);
            edges.push(Edge { u: u, v: v });
        }

        let mut ok = false;
        let mut visited = vec![false; n + 1];
        let mut parent: Vec<usize> = vec![0; n + 1];

        for i in 1..n + 1 {
            if visited[i] {
                continue;
            }
            let mut stack: Vec<usize> = Vec::new();
            stack.push(i);

            visited[i] = true;

            while stack.len() > 0 {
                let last = stack.pop().unwrap();
                for v in adj[last].iter() {
                    if visited[*v] {
                        continue;
                    }
                    visited[*v] = true;
                    stack.push(*v);
                    parent[*v] = last;
                }
            }
        }
        for i in 0..m {
            let mut parents: HashSet<usize> = HashSet::new();
            let edge = edges[i];
            let mut u = edge.u;
            parents.insert(u);
            while parent[u] != 0 {
                parents.insert(parent[u]);
                u = parent[u];
            }
            let mut v = edge.v;
            let mut common = usize::MAX;
            while parent[v] != 0 {
                if parents.contains(&parent[v]) && v != edge.u && !parents.contains(&v) {
                    common = parent[v];
                    break;
                }
                v = parent[v];
            }

            if common != usize::MAX {
                if common == edge.u || common == edge.v {
                    continue;
                }
                let mut nodes: Vec<usize> = Vec::new();
                nodes.push(edge.u);
                let mut u = edge.u;
                while parent[u] != common {
                    nodes.push(parent[u]);
                    u = parent[u];
                }
                nodes.reverse();
                nodes.push(edge.v);
                let mut v = edge.v;
                while parent[v] != common {
                    nodes.push(parent[v]);
                    v = parent[v];
                }
                nodes.push(common);
                let mut ans: Vec<Edge> = Vec::new();
                let mut nodes_map: HashSet<usize> = HashSet::new();
                for j in 0..nodes.len() {
                    nodes_map.insert(nodes[j]);
                }
                for j in 0..nodes.len() {
                    if degree[nodes[j]] > 3 {
                        for v in adj[nodes[j]].iter() {
                            if !nodes_map.contains(v) {
                                ans.push(Edge { u: nodes[j], v: *v });
                            }
                            if ans.len() == 2 {
                                break;
                            }
                        }
                        break;
                    }
                }
                if ans.len() > 1 {
                    for j in 0..nodes.len() {
                        ans.push(Edge {
                            u: nodes[j],
                            v: nodes[(j + 1) % nodes.len()],
                        });
                    }
                    ok = true;
                    println!("YES");
                    println!("{}", ans.len());
                    for j in 0..ans.len() {
                        println!("{} {}", ans[j].u, ans[j].v);
                    }
                    break;
                }
            }
        }
        if !ok {
            println!("NO");
        }
    }
}
