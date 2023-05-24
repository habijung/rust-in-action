fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // 수명 변수 'a, 'b
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;

    // 함수 호출 시에는 수명 애너테이션 생략 가능
    let res = add_with_lifetimes(&a, &b);

    println!("{}", res)
}
