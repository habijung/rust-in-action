# Chapter 2

## Directory 구조

```
chapther2
  - src
    - bin
      - {example}.rs
    - main.rs
  - Carge.lock
  - Cargo.toml
```

## 각 예제 실행 방법

각 챕터별 Cargo 프로젝트를 하나만 생성하고 싶어서, 내부 `bin` 폴더에서 Rust 파일을 각자 실행하는 방향으로 파일을 구성함.

파일 확장자 `.rs`를 빼고 실행하면 됨.

```shell
cd chapter2
cargo run --bin {example}
```
