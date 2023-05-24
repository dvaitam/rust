use std::io;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n: usize = line.trim().parse().expect("not int");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let a: Vec<usize> = line
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut mi: Vec<usize> = vec![0; n + 4];
    let mut mx: Vec<usize> = vec![0; n + 4];
    for i in 0..n {
        mi[a[i]] += 1;
        mx[a[i]] += 1;
    }
    let mut mx_ans = 0;
    for i in 0..n + 2 {
        if mx[i] > 0 {
            mx_ans += 1;
        }
    }
    let mut mi_ans = mx_ans;
    let mut last_zero = 0;
    let mut excess = 0;
    for i in 1..n + 2 {
        if mx[i] == 0 {
            if excess > 0 {
                if mx[last_zero] == 0 {
                    mx[last_zero] += 1;
                    mx_ans += 1;
                    excess -= 1;
                }
            }
            if excess > 0 {
                mx[i] += 1;
                mx_ans += 1;
            }
            excess = 0;
            last_zero = i;
        } else {
            excess += mx[i] - 1;
        }
    }
    //write(f, mx, "\n")
    let mut i = 1;
    while i <= n {
        if mi[i] > 0 && mi[i + 1] > 0 && mi[i + 2] > 0 {
            mi[i + 1] += mi[i];
            mi[i + 1] += mi[i + 2];
            mi[i] = 0;
            mi[i + 2] = 0;
            mi_ans -= 2;
            i += 2
        } else if mi[i] > 0 && mi[i + 2] > 0 {
            mi[i + 1] += mi[i];
            mi[i + 1] += mi[i + 2];
            mi[i] = 0;
            mi[i + 2] = 0;
            mi_ans -= 1;
            i += 2;
        } else if mi[i] > 0 && mi[i + 1] > 0 {
            mi[i] += mi[i + 1];
            mi[i + 1] = 0;
            i += 1;
            mi_ans -= 1;
        }
        i += 1;
    }
    println!("{} {}", mi_ans, mx_ans);
}
