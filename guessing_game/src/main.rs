use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏：");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你猜的数：");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
