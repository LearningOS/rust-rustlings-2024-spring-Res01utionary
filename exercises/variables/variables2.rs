// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    //编译器无法自动推断出x的类型 需要手动声明；
    //但与此同时Rust不允许使用未经初始化的变量 所以这里直接绑定一个值
    let x = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
