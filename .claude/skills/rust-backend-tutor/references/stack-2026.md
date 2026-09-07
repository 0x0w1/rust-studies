# Rust 백엔드 생태계 스냅샷 (2026-09 기준 — 답변 전 docs.rs / lib.rs 로 재확인)

## 사실상 표준 스택
| 역할 | 1순위 | 대안 | 메모 |
|---|---|---|---|
| async 런타임 | **tokio** 1.x | smol, async-std(사실상 유지 중단) | 생태계 대부분이 tokio 전제 |
| HTTP 프레임워크 | **axum** 0.8 | actix-web 4, rocket 0.5, poem, salvo | axum 은 tokio 팀 관리, tower 미들웨어와 호환. 0.7→0.8 에서 경로 문법 `/:id` → `/{id}` 로 바뀜 (옛 글 주의) |
| HTTP 저수준 | hyper 1.x | | axum/reqwest 의 기반 |
| 미들웨어 | tower, tower-http | | `Layer`/`Service` 추상화. CORS, trace, timeout, compression |
| 직렬화 | **serde** + serde_json | | 거의 모든 crate 가 `serde` feature 제공 |
| DB (SQL) | **sqlx** 0.8 (async, 컴파일 타임 검증) | diesel 2 (동기 ORM, 타입 강함), sea-orm (async ORM) | sqlx 는 `DATABASE_URL` 로 매크로 검증, 오프라인 모드 `cargo sqlx prepare` |
| 커넥션 풀 | sqlx 내장 `PgPool`/`SqlitePool` | deadpool, bb8 | |
| 에러 | thiserror 2 (라이브러리), anyhow 1 (앱) | eyre, snafu | |
| 로깅/트레이싱 | **tracing** + tracing-subscriber | log + env_logger | `RUST_LOG=debug` 필터, `#[instrument]` |
| 설정 | config, figment, dotenvy | | 12-factor: 환경변수 우선 |
| HTTP 클라이언트 | **reqwest** 0.12 | ureq(동기, 단순) | |
| 인증 | jsonwebtoken, argon2 (비밀번호 해시), axum-extra(쿠키), tower-sessions | | |
| 검증 | validator, garde | | |
| gRPC | tonic | | prost 로 protobuf 생성 |
| GraphQL | async-graphql | juniper | |
| WebSocket | axum 내장 `ws` feature, tokio-tungstenite | | |
| 메시지 큐/캐시 | redis (async), lapin(RabbitMQ), rdkafka | | |
| 테스트 | 내장 `#[tokio::test]`, axum `tower::ServiceExt::oneshot`, reqwest 로 E2E, testcontainers | | |
| 벤치 | criterion, oha / wrk (부하) | | |
| 개발 도구 | cargo-watch 또는 bacon, sqlx-cli, cargo-nextest | | 설치는 사용자가 `cargo install` |

## 배포/운영 배경
- 멀티스테이지 Dockerfile: `rust:1.xx` 빌더 → `debian:bookworm-slim` 또는 `gcr.io/distroless/cc` 런타임. `cargo chef` 로 의존성 레이어 캐시.
- 정적 링크: `x86_64-unknown-linux-musl` 타겟(설치는 사용자가 `rustup target add`) → `scratch` 이미지 가능. 단, OpenSSL 의존 crate 는 `rustls` feature 로 대체.
- 컴파일 시간 단축: `cargo check` 습관, `sccache`, `mold`/`lld` 링커, `[profile.dev] opt-level = 1` (의존성만) 등.
- 관측성: tracing → OpenTelemetry(`tracing-opentelemetry`), `/metrics` 는 `metrics` + `metrics-exporter-prometheus`.

## 개념 매핑 (다른 언어 경험자용)
| 개념 | Node/Express | Go | Java Spring | Rust/axum |
|---|---|---|---|---|
| 동시성 단위 | 이벤트 루프 + 콜백/Promise | goroutine(선점적 스케줄) | 스레드 / 가상 스레드 | tokio task (협력적, `.await` 에서만 양보) |
| 미들웨어 | `app.use(fn)` | `http.Handler` 래핑 | Filter/Interceptor | tower `Layer` |
| 의존성 주입 | 클로저/모듈 | 구조체 필드 | `@Autowired` | `State<Arc<AppState>>` 추출기 |
| 요청 파싱 | `req.body` | `json.Decode` | `@RequestBody` | `Json<T>`, `Path<T>`, `Query<T>` 추출기 (타입이 곧 검증) |
| 에러 | throw / next(err) | `error` 반환 | 예외 | `Result<T, E>` + `impl IntoResponse for E` |
| null | `undefined` | zero value / nil | `Optional` | `Option<T>` — 컴파일러가 처리 강제 |

## 함정
- async fn 안에서 `std::thread::sleep`, 동기 파일 I/O, CPU 무거운 계산 → 런타임 스레드 블로킹. `tokio::time::sleep`, `tokio::fs`, `spawn_blocking` 로.
- `Mutex` 가드를 `.await` 넘어서 들고 있으면 `Send` 에러 → **블록 스코프**로 가드를 `.await` 전에 끝내기(`{ let g = m.lock().unwrap(); ...; }`) 또는 `tokio::sync::Mutex`. 주의: `drop(guard)` 를 명시적으로 호출해도 컴파일러가 가드를 상태 머신에 남겨 에러가 그대로 난다(rust-lang/rust#57478) — 블록 스코프만 통한다.
- 핸들러 시그니처 에러(`Handler<_, _> is not implemented`)는 대개 추출기 순서(body 소비 추출기는 마지막) 또는 반환 타입이 `IntoResponse` 아님. `#[debug_handler]` 매크로로 진짜 원인 확인.
- axum 0.8 경로: `/users/{id}`. 옛 블로그의 `/users/:id` 는 컴파일은 되지만 런타임 패닉.
