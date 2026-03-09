use rand::RngExt;


fn main() {
    let op1 = rand::rng().random_range(0..100);
    let op2 = rand::rng().random_range(0..100);
    println!("{} + {} = ??", op1, op2);
    println!("??の値を入力して下さい:");
    let mut ans_input = String::new();
    std::io::stdin().read_line(&mut ans_input).unwrap();
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);
    if dbg!(ans_input == op1 + op2) {
        println!("正解！");
    } else {
        println!("不正解！");
    }
}
