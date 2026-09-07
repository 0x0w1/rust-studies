# 신뢰할 수 있는 최신 정보 출처

Rust는 6주 주기 릴리스. 버전/기능 여부는 반드시 확인 후 답하고 확인 날짜를 밝힌다.

## 공식 (1순위)
- The Book: https://doc.rust-lang.org/book/ · 한국어: https://doc.rust-kr.org/
- std 문서: https://doc.rust-lang.org/std/
- Reference(언어 명세): https://doc.rust-lang.org/reference/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Edition Guide(2021→2024 변화): https://doc.rust-lang.org/edition-guide/
- 릴리스 노트: https://doc.rust-lang.org/releases.html · https://blog.rust-lang.org/ · https://releases.rs/
- 에러 코드 색인: https://doc.rust-lang.org/error_codes/
- Clippy 린트 목록: https://rust-lang.github.io/rust-clippy/master/
- Rustonomicon(unsafe): https://doc.rust-lang.org/nomicon/
- Async Book: https://rust-lang.github.io/async-book/

## crate 정보
- https://docs.rs/<crate> — API 문서
- https://crates.io/crates/<crate> / https://lib.rs/<crate> — 버전, 다운로드, 대안 비교
- `cargo search <crate>` — 로컬에서 읽기 전용으로 최신 버전 확인 (허용됨)
- https://blessed.rs/crates — "이 용도엔 어떤 crate" 큐레이션

## 2026-09 기준 스냅샷 (답변 전 재확인 권장)
- stable 1.95 (2026-04-16). `cfg_select!` 안정화(cfg-if 대체), `match` 의 if-let guard 안정화, let chains는 1.88부터(edition 2024).
- 기본 에디션 2024 (`cargo new` 시). `unsafe extern`, `gen` 예약어, RPIT 라이프타임 캡처 규칙 변경 등이 2021과 다름.

## 검색 요령
- 기능 안정화 여부: `"<feature name>" site:blog.rust-lang.org` 또는 releases.rs 검색
- 에러 원인: 에러 코드 + 핵심 문구로 검색, users.rust-lang.org 답변 우선
- 커뮤니티 글은 작성 연도를 반드시 확인 — 2020년 이전 글은 에디션/async 관련 내용이 낡았을 확률이 높다
