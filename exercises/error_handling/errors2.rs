// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    //为了简化处理 Result 类型的错误传播，Rust 提供了 ? 运算符
    //它可以自动将 Err 的值提取并将整个函数的返回值设置为 Err，
    //从而使得错误可以被调用方处理，而无需手动编写 match 表达式来处理 Result。
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)

    //   不太优雅的写法 使用match处理
    //let processing_fee = 1;
    // let cost_per_item = 5;
    // let qty = item_quantity.parse::<i32>();
    // match qty{
    //     Ok(i) => Ok(i * cost_per_item + processing_fee),
    //     Err(msg) => Err(msg)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
