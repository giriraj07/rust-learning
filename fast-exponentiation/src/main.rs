use std::io;

fn fast_exp(x: i32, mut y: i32) -> i32 {
    let mut ans = 1;
    let mut pow = x;
    while y != 0 {
        if y & 1 == 1 {
            ans = ans * pow;
        }
        pow = pow * pow;
        y=y >> 1
    }
    return ans;
}

fn main() {
    let mut num = String::new();
    loop {
        println!("Enter a Number");
        io::stdin()
            .read_line(&mut num)
            .expect("Error while reading number");
        let mut x: i32 = num.trim().parse().expect("error");
        let mut num2 = String::new();
        num.clear();
        println!("Enter The power");
        io::stdin()
            .read_line(&mut num2)
            .expect("Error while reading number");

        let mut y: i32 = match num2.trim().parse() {
            Ok(num2) => num2,
            Err(_) => continue,
        };
        println!("{x} ^ {y} = {}", fast_exp(x, y));
    }
}
