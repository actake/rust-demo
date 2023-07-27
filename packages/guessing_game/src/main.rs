use std::{cmp::Ordering, io};

use rand::{thread_rng, Rng};

fn gen_random_number() -> u32 {
    let mut rng = thread_rng();

    rng.gen_range(1..=100)
}

/**
 * 猜数游戏：
 * 1. 生成随机数
 * 2. 读取命令行输入的数值
 * 3. 匹配二者的值，给出提示
 */
fn main() {
    println!("猜数游戏开始!");

    let random_number = gen_random_number();
    println!("Your random number is: {}", random_number);

    println!("随机数已生成，请输入你猜的数：");

    loop {
        let stdin = io::stdin();
        let mut input_num = String::new();
        stdin.read_line(&mut input_num).expect("读取行失败");
        let input_num = match input_num.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input_num.cmp(&random_number) {
            Ordering::Less => println!("小"),
            Ordering::Greater => println!("大"),
            Ordering::Equal => println!("BingGo!"),
        }
    }
}
