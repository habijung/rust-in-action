fn main() {
    let twenty = 20;         // 타입 추론
    let twenty_one: i32 = 21;     // 타입 애너테이션 사용
    let twenty_two = 22i32;  // 타입 접미사 사용

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;    // 단순 가독성 밑줄
    println!("{}", one_million.pow(2));  // 숫자는 메서드를 가질 수 있음

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);
}
