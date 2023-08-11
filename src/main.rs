
use std::io;

fn fact(x:i32) -> i32{
    let mut ans=1;
    for i in 2..=x{
        ans=ans*i;
    }
    return ans;
}

fn main() {
    let mut num=String::new();
    loop{
        println!("Enter a Number");
        io::stdin().read_line(&mut num).expect("Error while reading number");
        let x:i32= match num.trim().parse(){
            Ok(num)=> num,
            Err(_)=>continue,
        };
        
        println!("Factorial of a number = {}",fact(x));
    }
}