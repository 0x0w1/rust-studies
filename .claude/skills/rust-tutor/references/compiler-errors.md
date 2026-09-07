# 입문자가 자주 만나는 컴파일 에러와 코칭 방법

원칙: 에러를 **읽는 법**을 가르친다. 에러 메시지 구조는 ①`error[E0xxx]: 제목` ②`--> 파일:줄:칸` ③코드 조각과 `^^^` 표시 ④`help:`/`note:`. 대개 ④에 답이 있다. 항상 `rustc --explain E0xxx` 를 권한다.

## E0382 — use of moved value
- 원인: `String`, `Vec` 같은 힙 타입을 다른 변수/함수에 넘긴 뒤 다시 사용.
- 코칭: "이 값의 소유자가 지금 누구인가요?" → 해결 방향은 세 갈래: 참조(`&`)로 넘기기 / `clone()` / 함수가 값을 돌려주기. 입문자는 `clone()` 으로 도망치기 쉬우니 "왜 참조가 더 나은가"를 함께.
- 직접 만들어 볼 실험: `let s2 = s1; println!("{s1}");`

## E0499 — cannot borrow as mutable more than once / E0502 — mutable + immutable 동시
- 원인: 같은 스코프에서 `&mut` 이 둘, 또는 `&` 와 `&mut` 이 겹침.
- 코칭: "두 참조의 **마지막 사용 지점**이 어디인가요?" — NLL(Non-Lexical Lifetimes) 덕에 마지막 사용 이후엔 빌림이 끝난다. 순서를 바꾸는 것만으로 풀리는 경우가 많다.
- 대표 사례: `for x in &v { v.push(...) }`.

## E0308 — mismatched types
- 흔한 변형: `expected String, found &str` (→ `.to_string()` / `String::from`), `expected i32, found u32`, `expected (), found i32` (→ 함수 끝 `;` 문제).
- 코칭: "컴파일러가 기대한 타입과 실제 타입, 두 줄을 그대로 읽어 보세요."

## E0384 — cannot assign twice to immutable variable
- `let mut` 누락. 여기서 "왜 기본이 불변인가"를 얘기할 기회.

## E0277 — trait bound not satisfied
- 변형: `doesn't implement Debug` (→ `#[derive(Debug)]`), `cannot be formatted with {}` (→ `Display` 구현 또는 `{:?}`), `the ? operator can only be used in a function that returns Result`, `Rc<..> cannot be sent between threads`.
- 코칭: "어떤 트레이트가 필요하다고 하나요? 그 트레이트가 하는 일은?"

## E0106 — missing lifetime specifier
- 참조를 반환하는 함수에서 입력 참조가 둘 이상일 때.
- 코칭(10장 전이면): "지금은 반환하는 참조가 어느 인자에서 왔는지 컴파일러가 모른다는 뜻이에요. 10장에서 표기법을 배웁니다. 당장은 `String` 을 반환하게 바꿔 보세요."

## E0425 — cannot find value / E0433 — failed to resolve
- 오타, `use` 누락, 스코프 밖. `use std::io;` 처럼 모듈을 안 가져온 경우가 2장에서 잦다.

## E0599 — no method named ... found
- 트레이트가 스코프에 없음(`use rand::Rng`), 또는 타입 착오(`Option<T>` 에 `T` 메서드 호출 → `match`/`if let`/`unwrap`).

## E0004 — non-exhaustive patterns
- `match` 에 빠진 가지. `_` 로 뭉개기 전에 "빠진 경우가 실제로 무슨 상황인지" 물어보기.

## E0433 / E0432 — unresolved import (외부 crate)
- `Cargo.toml` `[dependencies]`에 없음. **AI 튜터가 `cargo add`하지 않는다.** "Cargo.toml [dependencies] 아래에 `rand = "0.9"`를 추가하세요"처럼 안내. 최신 버전은 `cargo search rand`로 함께 확인.

## 경고(warning)도 교재다
- `unused variable` → `_` 접두 의미
- `variable does not need to be mutable`
- `unused Result that must be used` → 에러 처리를 잊었다는 신호, 9장 예고

## 런타임 패닉
- `index out of bounds`, `called Option::unwrap() on a None value`, `attempt to subtract with overflow`(디버그 빌드만).
- `RUST_BACKTRACE=1 cargo run` 을 직접 실행해 보게 한다.
