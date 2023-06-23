fn main() {
    let a = 42;
    let r = &a;    // r is a reference to a (참조: 메모리 주소)
    let b = a + *r; // *r is the value of a  (역참조: 데이터)

    println!("a + a = {}", b);
}
