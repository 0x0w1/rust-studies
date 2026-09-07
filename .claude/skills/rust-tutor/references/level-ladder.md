# 레벨 사다리 — 입문 → 중급 → 고급 판단 기준과 커리큘럼

## 레벨 판단 신호
| 레벨 | 질문/코드에서 보이는 신호 | 설명 방식 |
|---|---|---|
| 입문 | `let mut` 헷갈림, `String` vs `&str`, E0382/E0502 를 처음 봄, `unwrap()` 남발, `clone()` 으로 도망 | 비유 → 최소 예제 → 예측 질문. 한 개념씩. 완성 코드 금지 |
| 중급 | 트레이트 바운드 설계 질문, 라이프타임 표기 실전(`'a` 여러 개, 구조체에 참조), 이터레이터 어댑터 조합, `Box<dyn Error>` vs 커스텀 에러, 모듈/워크스페이스 구조, async 첫 진입 | 대안 2~3개와 트레이드오프. std 소스·docs.rs 로 직접 확인 유도. Clippy `pedantic` 켜 보기 |
| 고급 | unsafe 정당화, `Pin`, `PhantomData`, variance, HRTB(`for<'a>`), GAT, proc-macro, `Ordering::Acquire` 등 메모리 모델, `#[repr]`, FFI 경계, 성능 프로파일링, 컴파일 시간 최적화 | 사양(Reference, Nomicon)을 함께 읽고 **검증 도구**(miri, loom, criterion, cargo-asm)로 확인하게 함. "확신 대신 측정" |

## 진단과 승급 — 말이 아니라 수행으로 판정

각 단계에서 아래 네 증거 중 최소 세 가지를 서로 다른 과제로 확인한다. 같은 예제를 외워 반복한 것은 한 가지 증거로만 센다.

1. **설명**: 핵심 불변식과 "왜"를 자기 말로 설명한다.
2. **예측**: 컴파일 결과, 소유권 이동, 런타임 상태 또는 출력 형태를 실행 전에 예측한다.
3. **변형**: 조건 하나를 바꾼 작은 문제에 개념을 전이한다.
4. **진단**: 실패한 코드나 명령에서 원인과 확인 방법을 스스로 찾는다.

| 도달 수준 | 통과 증거 | 통과시키면 안 되는 경우 |
|---|---|---|
| 입문 → 중급 준비 | ownership/borrowing, `Option`/`Result`, 기본 에러 네 종류를 설명·예측·수정 | `clone`, `unwrap`, 컴파일러 제안을 이유 없이 복사 |
| 중급 → 고급 준비 | API 대안을 트레이드오프로 비교하고, 라이프타임·트레이트·동시성 실패를 최소 재현으로 진단 | 실행 성공만 제시하고 안전성·테스트·실패 경로를 설명 못함 |
| 고급 | unsafe 불변식, 메모리/동시성 모델, 성능 주장을 사양과 도구로 검증하고 반례를 다룸 | `unsafe`를 "빠르게 하는 문법"으로 설명하거나 벤치마크 없이 성능 단정 |
| OS/시스템 진입 | 유저/커널 경계, fd/주소/스레드 소유권, syscall 또는 I/O 흐름을 그림과 관찰 결과로 연결 | 명령을 암기하지만 경계·상태 전이·실패 모드를 설명 못함 |

오답이면 정답을 바로 공개하지 않는다. 가장 작은 반례나 관찰 명령으로 되돌아간 뒤 새 변형 문제로 다시 확인한다. 한 번 맞힌 결과만으로 레벨을 올리지 않는다.

## 입문 커리큘럼 (Book 1~9장)
`book-roadmap.md` 참고. 목표: 소유권 모델을 그림 없이 설명할 수 있고, E0382/E0499/E0502/E0308 을 스스로 고칠 수 있고, `Option`/`Result` 를 `match`/`if let`/`?` 로 다룰 수 있다.

## 중급 커리큘럼 (Book 10~21장 + 실전)
1. **트레이트 설계** — 제네릭 vs 트레이트 객체(단형화 vs 동적 디스패치), 객체 안전성, 블랭킷 impl, `From`/`Into`/`AsRef`/`Borrow` 관용구, `impl Trait` 인자/반환, orphan rule 과 newtype.
2. **라이프타임 실전** — 구조체에 참조 담기, 생략 규칙, `'static` 의 두 의미(타입 바운드 vs 참조), 왜 "라이프타임을 늘릴 수 없는지", self-referential 구조체가 불가능한 이유(→ Pin 예고).
3. **이터레이터와 클로저** — `impl Iterator` 반환, 커스텀 `Iterator` 구현, `FnOnce/FnMut/Fn` 선택, `move` 와 캡처 규칙(2021 disjoint capture), 지연 평가와 `collect` 타입 추론(`turbofish`).
4. **에러 설계** — thiserror(라이브러리)/anyhow(앱), 에러 계층, `?` 의 `From` 변환, 컨텍스트 붙이기, 패닉 정책(`unwrap` 이 정당한 경우).
5. **스마트 포인터·내부 가변성** — `Rc<RefCell>` vs `Arc<Mutex>` 선택, `Cell`, `OnceCell/LazyLock`, `Cow`, 순환 참조와 `Weak`.
6. **동시성** — `Send/Sync` 자동 유도 규칙, 스코프 스레드(`thread::scope`), 채널 패턴, `Mutex` 독 오염(poison), 데이터 경쟁 vs 경쟁 조건 구분.
7. **async 기초** — Future 는 게으르다, 런타임 필요성, `Send` 바운드가 튀어나오는 이유, `select!`/`join!`, 취소 안전성(cancellation safety), 블로킹 코드 격리(`spawn_blocking`).
8. **프로젝트 구조** — lib/bin 분리, 워크스페이스, feature flag, `pub(crate)`, 문서화 주석과 doctest, 통합 테스트, `cargo clippy -- -W clippy::pedantic`, `cargo fmt`, `cargo deny`/`cargo audit`.
9. **타입으로 설계** — newtype, typestate 패턴, 불변식을 타입에 넣기(`NonZeroU32`, `NonEmpty`), `#[must_use]`, 빌더 패턴.

검증 과제 예: "임의의 `Read` 를 받아 줄 단위로 파싱하는 제네릭 함수를 `impl Iterator<Item = Result<Record, ParseError>>` 로 반환" — 라이프타임·트레이트·에러·이터레이터가 한 번에 나온다.

## 고급 커리큘럼
`advanced-rust.md` 참고. 주제: unsafe 계약과 UB, raw pointer/aliasing 모델(Stacked/Tree Borrows), `Pin` 과 자기참조, Future 상태 머신 손으로 그리기, 실행기(executor) 직접 만들기, 메모리 모델과 원자적 연산, lock-free 자료구조 검증(loom), `PhantomData`·variance·drop check, HRTB/GAT, 선언적/절차적 매크로, FFI(`bindgen`/`cbindgen`, `#[repr(C)]`, 패닉 경계), 성능(프로파일링, 할당, 캐시, SIMD `std::simd`/`portable-simd`), 컴파일러 내부(MIR, `cargo asm`, LTO/PGO), `no_std` 라이브러리 설계.

## OS/시스템 레벨은 별도 스킬
시스템 콜, 파일 디스크립터, epoll/io_uring, 프로세스/시그널, 커널 모듈(Rust for Linux), eBPF(aya), OS 직접 만들기(blog_os) 는 `rust-systems-tutor` 스킬이 담당한다. 그쪽 질문이 오면 그 스킬을 함께 참조한다.
