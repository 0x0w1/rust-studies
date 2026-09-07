# 백엔드 학습 경로 — 직접 만들며 배우는 단계

각 단계는 사용자가 **직접** `cargo new` → 코드 작성 → 실행 → `curl` 확인. AI 튜터는 각 단계의 개념·확인 방법·막힐 때 힌트를 준다.

## 0단계 — 준비 운동 (The Book 12~13장 수준)
- 목표: 프레임워크 없이 HTTP 를 손으로 느끼기 (Book 21장 선행).
- `std::net::TcpListener` 로 8080 수신 → 요청 첫 줄 출력 → `HTTP/1.1 200 OK\r\n\r\nhello` 응답.
- 확인: `curl -v localhost:8080`. 브라우저로도.
- 배우는 것: HTTP 는 텍스트다, 블로킹 I/O 의 한계(동시 접속 2개 실험).

## 1단계 — 첫 axum 서버 (Book 17장 이후)
- 의존성: tokio(full), axum. 최신 버전은 `cargo search` 로.
- `GET /health` → `"ok"`. `#[tokio::main]` 이 하는 일 설명.
- 확인: `curl -i localhost:3000/health`
- 실험: 핸들러 안에 `std::thread::sleep(5s)` 넣고 두 창에서 동시에 curl → 왜 둘째가 기다리나? → `tokio::time::sleep` 으로 바꾸면?

## 2단계 — JSON 과 추출기
- serde(derive), serde_json 추가. `Json<T>` 반환/수신, `Path<u32>`, `Query<T>`.
- 잘못된 JSON 보내면 axum 이 자동으로 422/400 주는 것 관찰 → "타입이 검증이다".

## 3단계 — 공유 상태와 동시성
- `Arc<Mutex<Vec<Todo>>>` 를 `State` 로. 왜 `Arc` 인가(태스크가 여러 스레드), 왜 `Mutex` 인가.
- 실험: `Rc` 로 바꿔 보기 → `Send` 에러 읽기. `std::sync::Mutex` vs `tokio::sync::Mutex` 언제 뭐.

## 4단계 — 에러 타입
- `enum AppError` + `thiserror` + `impl IntoResponse`. 핸들러가 `Result<Json<T>, AppError>` 반환.
- `?` 가 `From` 을 타는 것(Book 9장 복습).

## 5단계 — 데이터베이스
- sqlx + SQLite(`sqlite:todos.db`) 로 시작. `sqlx-cli` 로 마이그레이션(설치는 사용자).
- `query_as!` 매크로 컴파일 타임 검증 체험 → 컬럼 이름 오타를 컴파일러가 잡는 순간.
- 다음: PostgreSQL 로 교체(Docker 로 로컬 실행), 커넥션 풀 개념.

## 6단계 — 미들웨어와 관측성
- tower-http `TraceLayer`, `CorsLayer`, `TimeoutLayer`. tracing-subscriber 로 로그. `RUST_LOG` 필터.
- 직접 미들웨어 하나(`middleware::from_fn`)로 요청 ID 붙이기.

## 7단계 — 인증
- 비밀번호 해시(argon2) → JWT 발급(jsonwebtoken) → 추출기로 인증 강제(`FromRequestParts` 구현).
- 보안 배경: 왜 bcrypt/argon2 인지, JWT 의 한계, 쿠키 vs 헤더.

## 8단계 — 테스트와 구조
- `tower::ServiceExt::oneshot` 으로 핸들러 단위 테스트, 통합 테스트는 실제 포트 + reqwest.
- 모듈 분리: `routes/`, `handlers/`, `models/`, `errors.rs`, `state.rs`. Book 7장 복습.

## 9단계 — 배포
- 멀티스테이지 Dockerfile, `--release`, 환경변수 설정, graceful shutdown(`axum::serve(...).with_graceful_shutdown`).
- 선택: musl 정적 빌드, 헬스체크, Prometheus 메트릭.

## 이후 갈래
- gRPC(tonic) / GraphQL(async-graphql) / WebSocket 실시간
- 백그라운드 잡, 메시지 큐, 캐시(redis)
- 임베디드 장비 ↔ 서버 연동 (MQTT 브로커 + rumqttc) → `rust-embedded-tutor` 와 연결

## 참고 자료
- axum 공식 예제: https://github.com/tokio-rs/axum/tree/main/examples
- tokio 튜토리얼: https://tokio.rs/tokio/tutorial (미니 Redis 만들기 — 매우 추천)
- Zero To Production in Rust (책, actix 기반이지만 개념 우수)
- sqlx: https://github.com/launchbadge/sqlx
- tower 개념: https://docs.rs/tower/latest/tower/ 의 "Service" 설명
