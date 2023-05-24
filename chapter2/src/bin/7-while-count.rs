use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    // 1초 동안 루프를 돌면서 카운트를 증가시키는 간단한 벤치마크 구현
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
