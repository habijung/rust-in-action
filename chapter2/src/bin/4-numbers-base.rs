fn main() {
    let three = 0b11;           // 0b: 2진수
    let thirty = 0o36;          // 0o: 8진수
    let three_hundred = 0x12C;  // 0x: 16진수

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}