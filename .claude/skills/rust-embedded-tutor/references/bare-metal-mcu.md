# 베어메탈 MCU (no_std) — 2026-09 스냅샷

리눅스 없이 칩 위에서 직접 실행. Rust 의 "제로 코스트 추상화 + 소유권으로 하드웨어 안전"이 가장 빛나는 영역이지만 입문 장벽도 가장 높다. The Book 을 최소 13장(클로저/이터레이터)까지, 가능하면 16장(동시성)까지 마친 뒤 권장.

## 계층 구조 (아래→위)
1. **PAC** (Peripheral Access Crate) — svd2rust 가 데이터시트(SVD)에서 생성한 레지스터 접근 코드. `stm32f4`, `rp2040-pac` 등. 날것.
2. **HAL** — PAC 위에 안전한 API. 핀을 타입으로 소유, typestate 로 모드 전환. `stm32f4xx-hal`, `rp-hal`, `esp-hal`, `nrf-hal`, 그리고 embassy 의 `embassy-stm32`/`embassy-rp`/`embassy-nrf`.
3. **embedded-hal 1.0** — 벤더 무관 트레이트(`OutputPin`, `I2c`, `SpiDevice`, `DelayNs`). 드라이버 crate 는 이 트레이트만 의존 → 어떤 칩·SBC 에서도 재사용. `embedded-hal-async` 는 async 버전.
4. **프레임워크/런타임** — embassy(async 실행기 + HAL + 타이머 + 네트워크), RTIC(인터럽트 기반 실시간 스케줄러), 또는 베어 루프.
5. **드라이버** — 센서·디스플레이 등. awesome-embedded-rust 목록.

## 실행기 선택
| | 베어 루프 + HAL | RTIC 2 | embassy |
|---|---|---|---|
| 모델 | 동기, 폴링/인터럽트 직접 | 인터럽트 우선순위 기반 태스크, 정적 분석으로 데드락 없는 자원 공유 | async/await 태스크, `.await` 에서 저전력 대기 |
| 장점 | 이해 쉬움 | 하드 실시간, 결정론적 | 코드가 일반 async Rust 와 유사, Wi-Fi/BLE/USB/네트워크 스택 내장 |
| 상태(2026) | 항상 유효 | 활발 | **사실상 기본 선택**. RP2040/2350, STM32 전 계열, nRF, ESP32(esp-hal 통해) 지원. crates.io 릴리스와 git main 차이 주의 → 예제는 해당 릴리스 태그로 볼 것 |

## 입문 보드 추천 기준
| 보드 | 아키텍처 | 장점 | 도구 |
|---|---|---|---|
| **Raspberry Pi Pico / Pico 2** (RP2040 / RP2350) | Cortex-M0+ / M33 + RISC-V | 저렴, 문서·embassy 지원 최상, 2개 사면 하나를 디버그 프로브로 | probe-rs, UF2 드래그앤드롭(프로브 없이도 가능) |
| micro:bit v2 (nRF52833) | Cortex-M4 | Discovery Book 공식 교재, 온보드 디버거·센서·LED 매트릭스 | probe-rs |
| STM32 Nucleo (F4/L4/H7) | Cortex-M4/M7 | 산업 표준, 온보드 ST-Link | probe-rs |
| ESP32-C3/C6 (RISC-V) | RISC-V | Wi-Fi/BLE, esp-rs 가 Espressif 공식 1급 SDK 로 승격. RISC-V 계열은 표준 툴체인으로 빌드(Xtensa 계열 ESP32/S3 는 별도 툴체인 `espup`) | espflash, probe-rs(일부) |

## 툴체인·도구 (설치는 사용자가)
- `rustup target add thumbv6m-none-eabi` (RP2040) / `thumbv8m.main-none-eabihf` (RP2350) / `thumbv7em-none-eabihf` (STM32F4, nRF52) / `riscv32imc-unknown-none-elf` (ESP32-C3)
- **probe-rs** (`cargo install probe-rs-tools`): 플래시·디버그·RTT 로그. OpenOCD 를 실질적으로 대체. `cargo embed`, `probe-rs run`.
- **defmt**: 로그 포맷을 호스트에서 처리 → 펌웨어 크기 최소. `defmt-rtt` + `panic-probe`.
- `cargo-binutils` (`cargo size`, `cargo objdump`), `flip-link` (스택 오버플로 감지).
- 프로젝트 골격: `cargo generate` 템플릿(`embassy` 예제, `rp-rs/rp2040-project-template`). 링커 스크립트 `memory.x`, `.cargo/config.toml` 의 `runner`/`rustflags`.

## no_std 에서 달라지는 것 (입문자가 놀라는 지점)
- `println!` 없음 → defmt / RTT / UART. `Vec`/`String` 은 `alloc` + 글로벌 할당자(`embedded-alloc`) 필요, 보통 `heapless::Vec<T, N>` 로 고정 크기.
- `main` 이 아님 → `#[entry]`(cortex-m-rt) 또는 `#[embassy_executor::main]`. `fn main() -> !` (반환하지 않음).
- panic 처리기를 직접 지정(`panic-probe`, `panic-halt`).
- 부동소수점: FPU 없는 코어(M0+)는 소프트웨어 에뮬레이션 → 느림. 정수 연산 습관.
- 인터럽트 핸들러와 공유 상태: `static` + `Mutex<RefCell<...>>`(cortex-m) 또는 embassy `Signal`/`Channel`. 여기서 The Book 15~16장이 그대로 쓰인다.
- 크기와 시간: `--release` 필수, `opt-level = "s"/"z"`, `lto = true`. `cargo size` 로 확인.

## Rust 가 임베디드에서 좋은 이유 (동기 부여용)
- 핀·주변장치를 **소유권**으로 표현 → 같은 UART 를 두 곳에서 초기화하는 버그가 컴파일 에러.
- typestate: `Pin<Input<PullUp>>` 에 `set_high()` 를 부르면 컴파일 에러 — 런타임 검사 0.
- `Result` 강제 → I2C NACK 같은 실패를 무시하기 어렵다.
- async(embassy) 로 RTOS 없이 멀티태스킹, 스택 하나, 힙 없음.

## 함정
- HAL crate 버전마다 API 격변(embedded-hal 0.2→1.0, embassy 마이너 버전). 예제 코드의 `Cargo.toml` 버전과 내 것을 맞추거나, **의존성을 핀 고정**하고 마이그레이션은 별도 작업으로.
- 플래시 잘못 쓰기(부트로더 영역)는 벽돌 위험 — `memory.x` 를 건드리기 전에 백업/복구 절차 확인. RP2040 은 BOOTSEL 로 항상 복구 가능해 입문에 안전.
- 디버그 프로브 없이 시작하면 "왜 안 되는지" 알 길이 없어 좌절 → LED 한 개라도 상태 표시, 가능하면 프로브 확보.

## 학습 경로
1. Discovery Book (micro:bit v2) 또는 Pico + embassy 예제 `blinky` — 빌드·플래시·RTT 로그까지 파이프라인 확보
2. 버튼 → 인터럽트/async 대기(`wait_for_falling_edge`)
3. 타이머·PWM(LED 밝기), UART 로 PC 와 통신
4. I2C 센서 + embedded-hal 드라이버 crate → SBC 에서 쓴 것과 같은 crate 임을 확인
5. embassy 태스크 여러 개 + `Channel` 로 통신
6. (ESP32/Pico W) Wi-Fi + MQTT → 서버 연동 (`rust-backend-tutor`)

## 출처
- Embedded Rust Book: https://docs.rust-embedded.org/book/ · Discovery: https://docs.rust-embedded.org/discovery/
- embassy: https://embassy.dev/ (Book 포함) · https://github.com/embassy-rs/embassy/tree/main/examples
- awesome-embedded-rust: https://github.com/rust-embedded/awesome-embedded-rust
- esp-rs: https://docs.espressif.com/projects/rust/ · https://github.com/esp-rs
- probe-rs: https://probe.rs/ · defmt: https://defmt.ferrous-systems.com/
- Rust Embedded WG 블로그/매트릭스 채널, 연간 "State of Embedded Rust" 글(작성 연도 확인)
