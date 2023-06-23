fn main() {
    let a = 10;     // 컴파일러가 타입 추론
    let b: i32 = 20;     // 변수 타입 명시
    let c = 30i32;  // 숫자 타입은 리터럴 형식에 타입 애너테이션을 붙일 수 있음
    let d = 30_i32; // 리터럴에 밑줄을 넣어도 됨 (단순 가독성)
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
