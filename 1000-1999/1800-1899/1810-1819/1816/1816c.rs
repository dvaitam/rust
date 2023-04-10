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
        let n: usize = line.trim().parse().expect("not int");
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let mut a : Vec<i64> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
       
        if n %2 == 1 {
            println!("YES");
        }else{
            for i in 1..n {
                (a[i-1], a[i]) = (0, a[i]-a[i-1]);
            }
            if a[n-1] >= 0 {
                println!("YES");
            }else{
                println!("NO");
            }
        }
    }

    
}
