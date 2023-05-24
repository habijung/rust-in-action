use std::convert::TryInto; // 2018 edition에서는 사용함.

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // 
    let b_ = b.try_into().unwrap(); // Result 타입을 반환함

    if a < (b as i32) {
        println!("Ten is less than one hundred.")
    }

    if a < b_ {
        println!("Ten is less than one hundred.")
    }
}
