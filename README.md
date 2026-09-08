# rust-studies

Rust를 공부하며 실행한 예제와 배운 내용을 날짜별로 기록하는 저장소입니다.

[학습 실행](#학습-실행) · [학습 목차](#학습-목차) · [Daily Learning Changelog](#daily-learning-changelog) · [저장소 운영](#저장소-운영)

## 학습 실행

각 Cargo 예제는 저장소 루트에서 다음과 같이 실행합니다.

```sh
cargo run --manifest-path Chapter01/hello_cargo/Cargo.toml
cargo run --manifest-path Chapter01/guessing_game/Cargo.toml
cargo run --manifest-path Chapter03/variables/Cargo.toml
cargo run --manifest-path Chapter03/array_variables/Cargo.toml
```

단일 파일 예제는 `rustc Chapter01/hello_world.rs`로 컴파일할 수 있습니다.

## 학습 목차

- [`Chapter01`](Chapter01): Rust 프로그램 실행, Cargo 프로젝트, 숫자 맞히기 게임
- [`Chapter03`](Chapter03): 변수 shadowing과 스코프, 튜플과 배열 같은 복합 타입

## Daily Learning Changelog

최신 기록이 위에 오도록 관리합니다.

### 2026-09-08

- [`Chapter03/array_variables`](Chapter03/array_variables)에서 배열 `[1, 2, 3, 4, 5]`의 원소를 표준 입력으로 받은 인덱스로 접근하고, 범위를 벗어난 인덱스가 컴파일이 아닌 실행 시점 패닉으로 잡히는 것을 확인했습니다.
- [`Chapter03/variables`](Chapter03/variables)에서 튜플 `(i32, f64, u8)`을 `let (x, y, z) = tup;`으로 구조 분해하고, `tup.0` 같은 인덱스 접근으로도 같은 값을 읽었습니다.
- [`Chapter03/variables`](Chapter03/variables)에서 같은 이름을 `let`으로 다시 바인딩하는 shadowing을 실습했습니다.
- 중첩 스코프의 `x`가 바깥 스코프의 `x`와 독립적으로 계산되고, 스코프가 끝나면 바깥 값이 다시 보이는 것을 확인했습니다.
- Rust 언어·백엔드·임베디드·시스템 학습을 지원하는 tutor 스킬을 Claude Code와 Codex에서 함께 사용할 수 있도록 정리했습니다.

### 2026-09-07

- [`hello_world.rs`](Chapter01/hello_world.rs)를 `rustc`로 직접 컴파일하며 Rust 프로그램의 기본 진입점과 `println!`을 확인했습니다.
- [`hello_cargo`](Chapter01/hello_cargo)로 Cargo 프로젝트의 기본 구조와 실행 흐름을 익혔습니다.
- [`guessing_game`](Chapter01/guessing_game)에서 가변 변수, 표준 입력, 문자열 파싱, `match`, `Ordering`, `loop`와 `break`, 외부 `rand` 크레이트를 실습했습니다.

## 저장소 운영

- Claude Code는 `jig@jig` 플러그인을 사용합니다.
- Codex는 [`.agents/skills`](.agents/skills)의 `jig-*` 스킬을 사용합니다.
- 두 환경의 릴리즈 등급은 동일한 [`.jig/versioning.md`](.jig/versioning.md)를 기준으로 판정합니다.
