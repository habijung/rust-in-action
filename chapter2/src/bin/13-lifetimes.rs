fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // 수명 변수 'a, 'b
    *i + *j
}

fn main() {
    println!("{}", add_with_lifetimes(&10, &20))
}
