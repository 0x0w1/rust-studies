# 임베디드 리눅스 SBC (Raspberry Pi, ODROID 등) — 2026-09 스냅샷

std 가 있으므로 일반 Rust 그대로. 차이는 ①ARM 타깃 빌드 ②커널 인터페이스로 하드웨어 접근 ③배포/서비스화.

## 보드별 메모
| 보드 | SoC/아키텍처 | 타깃 | 메모 |
|---|---|---|---|
| Raspberry Pi 4/5, Zero 2 W, 3(64bit OS) | BCM27xx, Cortex-A72/A76 | `aarch64-unknown-linux-gnu` | Pi 5 는 GPIO 컨트롤러가 RP1 칩으로 바뀌어 옛 mmap 방식 라이브러리가 깨짐 → chardev 방식 사용 |
| Raspberry Pi Zero/1 | ARMv6 | `arm-unknown-linux-gnueabihf` | 매우 느림, 크로스 컴파일 필수 |
| ODROID-C4, N2/N2+, M1/M1S, M2 | Amlogic S905X3/S922X, Rockchip RK3568/RK3588 | `aarch64-unknown-linux-gnu` | Hardkernel wiki 의 GPIO 핀맵(`wiringPi` 번호 ≠ 커널 라인 번호) 주의. `gpioinfo` 로 실제 라인 확인 |
| ODROID-XU4 | Exynos 5422, 32bit | `armv7-unknown-linux-gnueabihf` | |
| Orange Pi, Rock Pi, Jetson | 대부분 aarch64 | `aarch64-unknown-linux-gnu` | |

`uname -m` (aarch64 / armv7l) 과 `ldd --version` (glibc 버전) 을 장비에서 먼저 확인하게 한다.

## 빌드 전략 3가지
1. **보드에서 직접 빌드** — rustup 설치 후 `cargo build`. Pi 4/5, ODROID N2+/M1 급이면 작은 프로젝트는 충분. 가장 함정이 적음. 단점: 느림, 의존성 많으면 메모리 부족.
2. **크로스 컴파일** — 개발 머신(macOS)에서 빌드해 전송.
   - `cross` (https://github.com/cross-rs/cross): Docker 필요, `cross build --target aarch64-unknown-linux-gnu --release`. 가장 쉬움.
   - `cargo-zigbuild`: Zig 를 링커로, Docker 없이 glibc 버전 지정 가능(`--target aarch64-unknown-linux-gnu.2.31`).
   - 직접: `rustup target add ...` + 크로스 링커(`aarch64-linux-gnu-gcc`, macOS 는 homebrew `messense/macos-cross-toolchains`) + `.cargo/config.toml` 의 `[target.aarch64-unknown-linux-gnu] linker = "..."`.
   - **glibc 함정**: 개발 머신 빌드 환경의 glibc 가 장비보다 새로우면 `GLIBC_2.xx not found`. → 장비와 같은 배포판 Docker 이미지로 빌드하거나 zigbuild 로 버전 고정, 또는 `aarch64-unknown-linux-musl` 정적 링크.
3. **musl 정적 바이너리** — `*-unknown-linux-musl` 타깃. 어느 리눅스에서든 실행. 단점: 일부 crate(OpenSSL 등) 추가 설정, 성능 소폭 차이.

전송/실행: `scp target/aarch64-unknown-linux-gnu/release/app user@board:~/` → `ssh user@board ./app`. 반복 시 `rsync`, 또는 `cargo run` 의 runner 를 ssh 스크립트로 지정(`.cargo/config.toml` `runner`).

## 하드웨어 접근 crate (embedded-hal 1.0 기준)
| 인터페이스 | 커널 노드 | crate | 상태(2026-09) |
|---|---|---|---|
| GPIO | `/dev/gpiochipN` (chardev v2) | **gpiocdev** (권장, async 지원), gpio-cdev(구, v1 ABI) | sysfs `/sys/class/gpio` 는 커널에서 deprecated — 사용하지 말 것 |
| GPIO/I2C/SPI/PWM/UART 통합(Pi 전용) | | rppal | **2025-07 은퇴(retired)** — 유지보수 없음. 기존 글의 예제는 참고만 |
| I2C | `/dev/i2c-N` | i2cdev, 또는 **linux-embedded-hal**(embedded-hal 트레이트 구현) | `i2cdetect -y 1` 로 주소 스캔 (i2c-tools) |
| SPI | `/dev/spidevB.C` | spidev, linux-embedded-hal | |
| UART | `/dev/ttyAMA0`, `/dev/ttyUSB0` | serialport, tokio-serial | |
| PWM | `/sys/class/pwm` (sysfs, 아직 표준) | sysfs_pwm, linux-embedded-hal | 하드웨어 PWM 핀은 보드마다 제한 |
| 센서 드라이버 | | embedded-hal 1.0 기반 드라이버(bme280, sht4x, mpu6050 등 — 각자 확인) | HAL 0.2 전용 드라이버는 `embedded-hal-compat` 로 브릿지 가능 |

**linux-embedded-hal 의 의의**: `embedded_hal::i2c::I2c` 같은 트레이트를 리눅스 장치 파일로 구현 → MCU 용으로 쓰인 센서 드라이버 crate 가 SBC 에서도 그대로 동작. 이것이 SBC 에서 시작해 MCU 로 가는 다리.

## 권한과 설정
- GPIO: `gpio` 그룹(`sudo usermod -aG gpio $USER`) 또는 udev 규칙. `sudo` 로 돌리는 습관은 경계.
- I2C/SPI 활성화: Pi 는 `raspi-config` 또는 `/boot/firmware/config.txt` 의 `dtparam=i2c_arm=on`; ODROID 는 device tree overlay(Hardkernel wiki).
- 확인 도구: `gpiodetect`, `gpioinfo`, `gpioset`/`gpioget`(libgpiod 도구 — 코드 짜기 전에 손으로 LED 켜 보기), `i2cdetect`, `ls /dev/spidev*`.

## 배선 안전 (항상 먼저 말할 것)
- GPIO 는 **3.3V 로직**, 핀당 수 mA~16mA. 5V 신호 직결 금지(레벨 시프터). 모터/릴레이는 트랜지스터·드라이버 보드 경유.
- LED: 직렬 저항 220~330Ω. GPIO → 저항 → LED(+, 긴 다리) → LED(−) → GND.
- 버튼: 내부 풀업 사용(`Bias::PullUp`) 후 GND 로 당김. 누르면 Low.
- 전원 끄고 배선. 핀 번호는 **물리 핀 번호 vs BCM/커널 라인 번호** 혼동이 가장 흔한 사고.

## 서비스화·운영
- systemd unit(`/etc/systemd/system/app.service`, `Restart=on-failure`, `journalctl -u app -f`).
- 로그: tracing + journald. 설정: 환경변수(`EnvironmentFile=`).
- 업데이트: scp 후 `systemctl restart`. 규모가 커지면 `cargo-deb` 로 .deb 패키징.
- 네트워크 연동: MQTT(rumqttc, 브로커 mosquitto), HTTP(reqwest, rustls feature 로 OpenSSL 회피).

## 첫 프로젝트 순서(권장)
1. 장비에서 `gpioinfo` → LED 핀 결정 → `gpioset` 으로 손으로 켜기 (코드 0줄)
2. Rust: gpiocdev 로 1초 깜빡이기 (`std::thread::sleep`)
3. 버튼 입력 → 눌리면 LED 토글 (폴링) → 엣지 이벤트로 개선
4. I2C 온습도 센서 읽어 콘솔 출력 (`i2cdetect` → 드라이버 crate → 없으면 데이터시트 레지스터 직접)
5. 읽은 값을 HTTP/MQTT 로 서버에 전송 → `rust-backend-tutor` 로 연결
6. systemd 서비스로 부팅 시 자동 실행

## 출처
- Embedded Linux 와 Rust 개론: https://docs.rust-embedded.org/book/ (MCU 중심이지만 개념 공통)
- gpiocdev: https://docs.rs/gpiocdev · libgpiod 도구 문서
- linux-embedded-hal: https://github.com/rust-embedded/linux-embedded-hal
- cross: https://github.com/cross-rs/cross · cargo-zigbuild: https://github.com/rust-cross/cargo-zigbuild
- Raspberry Pi GPIO 핀아웃: https://pinout.xyz/ · ODROID: https://wiki.odroid.com/
