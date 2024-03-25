mod module{
   pub mod test; // 内部模块
}
use module::test::plus_one;

fn main() {
    let x = 5;
    let y = plus_one(x);
    println!("The value of y is: {}", y);

    println!("Hello, world!");
}
