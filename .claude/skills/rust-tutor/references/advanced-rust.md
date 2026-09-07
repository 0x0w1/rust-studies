# 고급 Rust — 주제별 핵심, 함정, 검증 도구 (2026-09 기준)

원칙: 고급 주제에서는 "정답을 아는 것"보다 **"어떻게 확인하는가"**를 가르친다. 모든 항목에 검증 도구를 붙였다.

## unsafe 와 UB
- `unsafe` 는 5가지만 허용: raw pointer 역참조, unsafe fn/메서드 호출, `static mut` 접근, unsafe 트레이트 impl, union 필드 접근. 나머지 검사는 그대로 동작한다.
- UB 목록(Reference "Behavior considered undefined"): 데이터 경쟁, dangling/unaligned 역참조, 잘못된 값(예: `bool` 에 2), aliasing 규칙 위반(`&mut` 이 유일하지 않음), 초기화 안 된 메모리 읽기 등.
- 안전 추상화 원칙: `unsafe` 블록 위에 **`// SAFETY:` 주석으로 어떤 불변식에 의존하는지** 적고, 그 불변식을 모듈 경계 안에서 보장한다. `pub` 안전 함수가 UB 를 일으킬 수 있으면 그 모듈은 unsound.
- aliasing 모델: Stacked Borrows → Tree Borrows 로 발전 중. `&mut T` 에서 만든 raw pointer 와 원래 참조를 섞어 쓰면 UB 가 될 수 있다.
- **검증**: `rustup +nightly component add miri` 후 `cargo +nightly miri test` (UB 검출기 — 고급 학습자의 필수 도구). Miri 플래그는 환경변수로: `MIRIFLAGS="-Zmiri-tree-borrows" cargo +nightly miri test` (`cargo miri test -Z...` 는 cargo 가 거부). 그 외 `cargo careful`, `-Zsanitizer=address`.

## Pin, Future, async 내부
- `async fn` 은 컴파일러가 만드는 상태 머신 `impl Future<Output=T>`. `.await` 지점마다 상태가 하나. 지역 변수가 상태 머신 필드가 되므로 참조를 `.await` 넘어 들고 있으면 **자기참조 구조체**가 된다 → 이동하면 깨짐 → `Pin` 이 "이 값은 더 이상 이동하지 않는다"를 타입으로 보장.
- `Unpin` 은 자동 트레이트(대부분 타입은 Unpin) — 자기참조 타입엔 `PhantomPinned` 필드를 넣어 `!Unpin` 으로 만들어야 `Pin` 이 의미를 가진다. `Unpin` 바운드가 있는 API: `Pin::into_inner`, `Pin::get_mut`, `DerefMut`. **없는** API: `Pin::set`(값 교체는 drop-in-place 후 쓰기라 안전), `Pin::as_mut`. `Pin<Box<T>>`, `pin!` 매크로, `Box::pin`.
- 실행기(executor): `Waker` 로 "다시 poll 해 달라"를 알린다. 손으로 200줄짜리 단일 스레드 실행기를 만들어 보면 모든 게 이해된다(Async Book, tokio 튜토리얼 "Async in depth").
- 취소 안전성: `select!` 에서 진 브랜치는 drop 된다 — 중간 상태가 유실되는 Future 가 있다(예: `read_exact`).
- `Send` 에러: `.await` 넘어 `MutexGuard`/`Rc`/`*mut` 를 들고 있음. `tokio::task::spawn_local`, 스코프 줄이기, `tokio::sync::Mutex`.
- **검증**: `tokio-console`, `tracing`, `cargo expand`(상태 머신은 안 보이지만 매크로 확인), 직접 만든 실행기.

## 메모리 모델과 원자적 연산
- C++20 모델 기반. `Relaxed`(순서 보장 없음, 카운터), `Acquire/Release`(락·플래그 핸드오프), `AcqRel`, `SeqCst`(전역 순서, 비쌈). "일단 SeqCst" 는 정답이 아니라 회피.
- happens-before 를 설명할 수 있어야 한다. 스핀락, 한 번 초기화(`Once`), SPSC 큐를 직접 구현해 보는 것이 최고의 연습.
- **검증**: `loom`(모든 인터리빙 탐색), miri(데이터 경쟁 감지 일부), 벤치마크로 Ordering 비용 비교.
- 책: Mara Bos, *Rust Atomics and Locks* (무료 온라인: https://marabos.nl/atomics/).

## 타입 시스템 심화
- **Variance**: `&'a T` 는 `'a` 에 공변, `&'a mut T` 는 `T` 에 불변, `fn(T)` 는 반변. 왜 `&mut Vec<&'static str>` 에 `&mut Vec<&'a str>` 를 못 넣는지 설명할 수 있어야 함.
- `PhantomData<T>`: 소유권/variance/drop check 를 컴파일러에 알려주는 0바이트 표식. raw pointer 래퍼에 필수.
- Drop check(`#[may_dangle]`), `ManuallyDrop`, `mem::forget` 과 leak 의 안전성(leak 은 safe!).
- HRTB `for<'a> Fn(&'a T) -> &'a U`: 클로저가 임의 라이프타임에 대해 동작해야 할 때. GAT(1.65+): `type Item<'a> where Self: 'a` — lending iterator.
- Coherence/orphan rule, 특수화 부재, `dyn Trait` 객체 안전성(제네릭 메서드·`Self` 반환 불가, `where Self: Sized` 우회), dyn upcasting(1.86+).
- **검증**: 컴파일러 에러 자체가 검증기. `cargo doc` 으로 자동 트레이트 impl 확인.

## 매크로
- 선언적 `macro_rules!`: fragment specifier(`expr`, `ident`, `tt`…), 반복 `$(...),*`, 위생(hygiene), 재귀 매크로, `tt muncher`. Little Book of Rust Macros.
- 절차적: `proc-macro = true` crate 분리, `syn`/`quote`/`proc-macro2`, derive/attribute/function-like 3종. 에러는 `syn::Error::to_compile_error`. 테스트는 `trybuild`.
- **검증**: `cargo expand`, `trybuild`(컴파일 실패 테스트), `macrotest`.

## FFI
- `extern "C"`, `#[repr(C)]`, `#[no_mangle]`(2024 에디션: `#[unsafe(no_mangle)]`), `CStr/CString`, `bindgen`(C→Rust), `cbindgen`(Rust→C 헤더), `cxx`(C++ 안전 브릿지), PyO3/napi-rs.
- 패닉은 FFI 경계를 넘으면 UB(→ `extern "C-unwind"` 또는 `catch_unwind`). 소유권 이전 규칙을 문서화(`Box::into_raw`/`from_raw` 짝).
- **검증**: valgrind/ASan, miri 는 FFI 불가, 소유권 테스트를 양쪽에서.

## 성능
- 순서: 측정(criterion, `hyperfine`) → 프로파일(`cargo flamegraph`, `perf`, `samply`, macOS Instruments) → 할당 추적(`dhat`, `heaptrack`) → 그 다음에 최적화.
- 흔한 원인: 불필요한 `clone`/`String` 할당, `Vec` 재할당(`with_capacity`), `Box<dyn>` 남발, 작은 구조체의 간접 참조, `HashMap` 기본 해셔(SipHash → `ahash`/`FxHash`), 경계 검사(이터레이터가 더 빠른 경우), 디버그 빌드로 측정(!).
- 빌드: `--release`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, PGO(`cargo-pgo`), `target-cpu=native`(배포 주의).
- SIMD: `std::simd`(nightly) / `wide`, 자동 벡터화 확인은 `cargo asm`/Compiler Explorer.
- **검증**: 벤치마크 없이 "더 빠를 것" 이라는 주장은 받아들이지 않는다.

## 컴파일러와 빌드 내부
- 파이프라인: 파싱 → HIR → THIR → MIR(borrow check 는 MIR 위) → LLVM IR → 기계어. `cargo rustc -- -Zunpretty=mir`(nightly), `cargo asm`, `cargo llvm-lines`(단형화 폭발 진단).
- 컴파일 시간: `cargo build --timings`, 의존성 feature 다이어트, 제네릭 → `dyn` 또는 내부 비제네릭 함수 분리, `mold`/`lld`, `sccache`, `cranelift` 백엔드(디버그).
- **검증**: `--timings` HTML, `cargo bloat`.

## no_std 라이브러리 설계
- `#![no_std]` + `extern crate alloc` 선택, `core::fmt`, feature 로 `std` 옵션 제공, `heapless`, `embedded-hal` 트레이트 의존. `rust-embedded-tutor` 와 연결.

## 참고
- Rustonomicon https://doc.rust-lang.org/nomicon/ · Reference UB 목록 · Rust Atomics and Locks · Little Book of Rust Macros https://veykril.github.io/tlborm/ · Async Book · Jon Gjengset *Rust for Rustaceans*, Crust of Rust 영상 · Unsafe Code Guidelines WG · Miri https://github.com/rust-lang/miri
