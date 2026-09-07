# The Rust Book 로드맵 — 장별 핵심과 입문자 함정

저장소 구조는 `ChapterNN/<project>` 이다. 사용자가 어디쯤인지 파악할 때 참고한다.
공식 원문: https://doc.rust-lang.org/book/ (한국어 번역: https://doc.rust-kr.org/)

| 장 | 주제 | 반드시 체득할 것 | 흔한 함정 / 코칭 포인트 |
|---|---|---|---|
| 1 | 설치, Hello World, Cargo | `rustc` vs `cargo`, `cargo new/build/run/check`, `Cargo.toml` 구조 | `rustc file.rs` 산출물이 소스 옆에 생김(.gitignore 참고). `cargo check` 가 빠른 이유 |
| 2 | 추리 게임 | `let mut`, `String::new()`, `read_line`, `match`, `Result`, crate 추가 | `.expect()` 는 임시방편임을 짚기. `use rand::Rng` 가 왜 필요한지(trait 메서드) |
| 3 | 변수, 타입, 함수, 제어문 | 불변 기본, 섀도잉 vs mut, 표현식 vs 문장, 튜플/배열 | `;` 유무로 반환값이 바뀌는 것. 정수 오버플로 디버그/릴리스 차이 |
| 4 | **소유권** | move, `Copy` vs `Clone`, 참조 `&`/`&mut`, 슬라이스 `&str` vs `String` | 가장 큰 고비. E0382(moved value), E0499/E0502(borrow 충돌)를 **직접 만들어 보게** 유도 |
| 5 | 구조체 | `impl`, 메서드 vs 연관 함수, `#[derive(Debug)]`, `{:?}`/`{:#?}` | `self` / `&self` / `&mut self` 차이를 4장과 연결 |
| 6 | 열거형, 패턴 매칭 | `Option<T>`, `match` 완전성, `if let` | null 대신 `Option` 을 쓰는 이유. `unwrap()` 습관 경계 |
| 7 | 모듈 시스템 | `mod`, `pub`, `use`, 파일 분리, crate root | `mod foo;` 선언과 파일 위치 관계. 2018+ 경로 규칙 |
| 8 | 컬렉션 | `Vec`, `String`(UTF-8), `HashMap` | 문자열 인덱싱 불가 이유. `entry().or_insert()` 패턴. 순회 중 push 시 borrow 에러 |
| 9 | 에러 처리 | `panic!`, `Result`, `?`, 에러 전파, `Box<dyn Error>` | `?` 는 `From` 변환을 한다는 것. 언제 panic이 정당한지 |
| 10 | 제네릭, 트레이트, 라이프타임 | `impl<T>`, 트레이트 바운드, `'a` 표기 | 라이프타임은 "검사기에게 관계를 알려주는 것"이지 수명을 늘리는 게 아님. 생략 규칙 3가지 |
| 11 | 테스트 | `#[test]`, `assert!`, `cargo test`, 통합 테스트 | `tests/` 디렉터리는 라이브러리 crate 대상. `--nocapture` |
| 12 | CLI 프로젝트(minigrep) | 인자 파싱, 파일 I/O, 리팩터링, 환경변수 | 앞 장 총정리. 여기서 `lib.rs`/`main.rs` 분리 배움 |
| 13 | 클로저, 이터레이터 | `Fn/FnMut/FnOnce`, `iter/into_iter/iter_mut`, 어댑터, 지연 평가 | 이터레이터는 **소비자**(`collect`, `sum`)가 있어야 실행됨. `move` 클로저 |
| 14 | Cargo 심화 | 프로파일, 문서화 주석, workspace, crates.io | `cargo doc --open` 을 꼭 해보게 |
| 15 | 스마트 포인터 | `Box`, `Rc`, `RefCell`, `Deref`, `Drop`, 순환 참조 | 런타임 borrow 검사(`RefCell`)와 컴파일타임 검사 차이. `Weak` |
| 16 | 동시성 | `thread::spawn`, `move`, 채널, `Mutex`, `Arc`, `Send/Sync` | `Rc` 가 왜 스레드에 못 넘어가는지 — 컴파일러가 알려주는 경험 |
| 17 | async/await | `Future`, `.await`, 런타임(tokio 등), `join!`/`select!` | 이 장은 backend 스킬(`rust-backend-tutor`)과 연결 |
| 18 | OOP 관점 | 트레이트 객체 `dyn Trait`, 상태 패턴 | 상속 없음. 정적/동적 디스패치 비용 |
| 19 | 패턴 | 구조 분해, 가드, `@` 바인딩, refutable/irrefutable | |
| 20 | 고급 | `unsafe`, 고급 트레이트/타입, 함수 포인터, 매크로 | `unsafe` 는 "검사를 끄는 것"이 아니라 "내가 책임진다" 선언 |
| 21 | 멀티스레드 웹 서버 | TCP, 스레드 풀, graceful shutdown | backend 스킬로 자연스럽게 이어지는 지점 |

## 장 사이에 곁들이면 좋은 자료
- Rust by Example: https://doc.rust-lang.org/rust-by-example/ — 각 개념의 짧은 실행 예제
- Rustlings: https://github.com/rust-lang/rustlings — 컴파일 에러 고치기 연습 (이 저장소 철학과 딱 맞음)
- `rustc --explain EXXXX` — 에러 코드 공식 설명
- 4장 이후: https://rust-unofficial.github.io/too-many-lists/ (소유권 심화, 15장쯤 추천)

## 진행 진단 질문 예시
- "지금 `cargo run` 하면 어떤 출력이 나오나요?"
- "이 변수를 한 번 더 사용하면 컴파일러가 뭐라고 할 것 같아요?"
- "이 함수가 `String` 을 받게 하면 호출하는 쪽에서 무슨 일이 생길까요?"
