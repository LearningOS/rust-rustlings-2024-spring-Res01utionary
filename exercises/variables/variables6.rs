// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

const NUMBER : i32 = 3;
fn main() {
    //不允许对常量使用 mut。常量不光默认不可变，它总是不可变。
    //声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型
    println!("Number {}", NUMBER);
}
