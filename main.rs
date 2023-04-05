use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    let number : i32 = args[1].parse().expect("Enter number");
    let mut num = number;
    let thousands = (num/1000)*1000;
    let t_dir = format!("{}-{}", thousands, thousands+999);
    num = num % 1000;
    let hundreds = (num/100)*100;
    let h_dir = format!("{}-{}", thousands+hundreds, thousands+hundreds+99);
    num = num % 100;
    let tens = (num/10)*10;
    let te_dir = format!("{}-{}", thousands+hundreds+tens, thousands+hundreds+tens+9);
    let c_dir = format!("{}", number);
    let file_name = c_dir.clone()+&args[2];
    let final_path = format!("{}/{}/{}/{}/{}.rs", t_dir, h_dir, te_dir, c_dir,file_name);
    println!("{}", final_path);
    let path = std::path::Path::new(&final_path);
    let prefix = path.parent().unwrap();
std::fs::create_dir_all(prefix).unwrap();
let data = "";
    std::fs::write(&final_path, data).expect("Unable to write file");

}
