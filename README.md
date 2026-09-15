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
cargo run --manifest-path Chapter03/functions/Cargo.toml
cargo run --manifest-path Chapter03/if_else/Cargo.toml
cargo run --manifest-path Chapter04/owner/Cargo.toml
cargo run --manifest-path Chapter04/references/Cargo.toml
```

단일 파일 예제는 `rustc Chapter01/hello_world.rs`로 컴파일할 수 있습니다.

## 학습 목차

- [`Chapter01`](Chapter01): Rust 프로그램 실행, Cargo 프로젝트, 숫자 맞히기 게임
- [`Chapter03`](Chapter03): 변수 shadowing과 스코프, 튜플과 배열 같은 복합 타입, 함수와 표현식·구문의 차이, `if`·`loop`·`while`·`for` 제어 흐름
- [`Chapter04`](Chapter04): 스택과 힙, 소유권 이동과 `clone`, 함수 호출과 반환에서의 소유권, 참조자와 빌림, 가변 참조자

## Daily Learning Changelog

최신 기록이 위에 오도록 관리합니다.

### 2026-09-15

- [`Chapter04/owner`](Chapter04/owner)에서 `let s = String::from("hello");`와 `let mut s = ...`의 차이를 확인하고, 불변 바인딩에 `push_str`을 호출하면 `` error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable ``가, 바꾸지 않는 값을 `mut`으로 선언하면 `warning: variable does not need to be mutable`가 나오는 것을 확인했습니다.
- `let s2 = s1;`이 힙 데이터를 복사하지 않고 소유권만 옮기는 이동(move)이라는 것을 확인하고, 이동 후 원본을 쓰면 `error[E0382]: borrow of moved value`가 나는 것을 주석으로 남겼습니다.
- `s3.clone()`으로 깊은 복사를 만든 뒤 `s3.as_ptr()`과 `s4.as_ptr()`을 `{:p}`로 출력해 힙 주소가 서로 다른 것을 눈으로 확인했습니다. 이동은 힙 주소가 그대로라는 점과 비교했습니다.
- `gives_ownership`과 `takes_and_gives_back`으로 함수 호출·반환에서 소유권이 어떻게 옮겨 다니는지 실습하고, 함수로 넘긴 `s2`를 다시 쓰면 `` error[E0382]: borrow of moved value: `s2` ``가 나는 것을 확인했습니다.
- [`Chapter04/references`](Chapter04/references)에서 `calculate_length(&s1)`로 소유권을 넘기지 않고 길이만 읽어, 튜플로 값을 되돌려 받던 패턴이 필요 없어지는 것을 확인했습니다.
- `change(&mut s2)`로 빌린 값을 변경하려면 원본이 `mut`이어야 하고 파라미터 타입도 `&mut String`이어야 한다는 것을 확인했습니다.
- 러스트의 `Copy`·`Clone`·이동을 Python의 얕은 복사·깊은 복사, Java·Kotlin의 참조 복사와 대조해 정리했습니다.

### 2026-09-10

- [`Chapter03/functions`](Chapter03/functions)에서 매개변수가 있는 함수 `print_labeled_measurement(value: i32, unit_label: char)`를 정의하고 호출했습니다.
- 블록 `{ let x = 3; x + 1 }`이 마지막 표현식의 값을 갖는 표현식이라는 것을 확인하고, `x + 1` 뒤에 세미콜론을 붙이면 구문(statement)으로 바뀌어 블록 값이 `()`가 되면서 `println!`에서 `` `()` doesn't implement `std::fmt::Display` `` 오류가 나는 이유를 정리했습니다.
- `fn five() -> i32 { 5 }`와 `fn plus_one(x: i32) -> i32 { x + 1 }`처럼 마지막 표현식이 반환값이 되는 함수를 작성하고, `5;`처럼 세미콜론을 붙이면 `mismatched types: expected i32, found ()` 오류가 나는 것을 확인했습니다.
- C/C++의 세미콜론은 문장 종결자일 뿐이지만 Rust의 세미콜론은 표현식의 값을 버려 구문으로 만들기 때문에 유무에 따라 타입이 달라진다는 차이를 비교했습니다.
- [`Chapter03/if_else`](Chapter03/if_else)에서 `if`/`else` 분기와 `let number = if condition { 5 } else { 6 };`처럼 `if`를 표현식으로 써서 값을 바인딩하는 방법을 실습하고, 분기 타입이 다르면(`5`와 `"six"`) 컴파일 오류가 나는 것을 확인했습니다.
- `loop`에서 `break counter * 2;`로 반복문 자체가 값을 반환하게 하고, `'counting_up` 라벨로 중첩 반복문에서 바깥 `loop`를 한 번에 빠져나오는 것을 확인했습니다.
- `while`로 카운트다운을 작성하고, 배열 순회를 `while`과 인덱스로 구현한 뒤 `for element in a`로 바꿔 비교했으며, `(1..4).rev()` 범위를 `for`로 역순 순회했습니다.

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
