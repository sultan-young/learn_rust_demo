use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜测一个数");

    // 生成一个随机整数，区间为 [1- 101)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("私密数字为{}", secret_number);
    
    loop {
        let mut guess = String::new();

        // 读取用户的输入
        io::stdin().read_line(&mut guess).expect("无法读取");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("您猜测的数为 {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
