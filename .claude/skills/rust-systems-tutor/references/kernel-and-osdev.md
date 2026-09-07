# 커널 모듈(Rust for Linux), eBPF(aya), OS 직접 만들기 — 2026-09 스냅샷

**모든 실습은 VM에서.** 커널 패닉·잘못된 eBPF·부트로더 실험은 호스트를 멈출 수 있다. 스냅샷을 찍고 시작한다. AI 튜터는 `insmod`, `bpftool`, `dd` 같은 명령을 대신 실행하지 않는다.

## A. Rust for Linux — 커널 모듈/드라이버
### 상태 (2026-09, 재확인 필요)
- Linux 7.0(2026-04) 에서 Rust 지원이 experimental 표기를 벗음. 안정 Rust 로 빌드 — 최소 버전은 `Documentation/rust/` version policy 에서 확인(2026-09 기준 문서상 1.85.0). 메인라인 안: Android Binder(Rust 구현), Nova(NVIDIA GPU, 진행 중), 여러 PHY/platform 드라이버, 블록 계층 추상화. 메인라인 밖(out-of-tree): Asahi GPU(DRM), NVMe 시제품 등 — 어느 쪽인지는 https://rust-for-linux.com/ 의 목록으로 확인. 커널의 대부분은 C 로 남고 **새 코드**에 Rust.
- `kernel` crate API 는 여전히 릴리스마다 변함 → 대상 커널 트리의 `rust/kernel/` 와 `samples/rust/` 가 1차 출처. 외부 블로그 코드는 커널 버전 확인 필수.
- 지원 아키텍처: x86_64, arm64, riscv64, loongarch, (일부) 32bit arm. `make LLVM=1` 권장.

### 전제 지식
- Rust: unsafe 계약, `Pin`(커널 객체는 대부분 이동 불가 → `pin-init`), 트레이트 객체, `no_std`.
- 커널: C 로 hello 모듈 한 번은 해봤을 것(LDD3 1~3장), 빌드 시스템(Kbuild), `dmesg`, `modprobe`.

### 첫 파이프라인 (사용자가 직접)
1. VM 에 커널 소스(대상 버전) + `rustup component add rust-src` + `bindgen-cli`(`cargo install --locked bindgen-cli` — 버전은 `Documentation/rust/quick-start.rst` 확인)
2. `make LLVM=1 rustavailable` → "Rust is available!" 이 나와야 함
3. `make LLVM=1 menuconfig` → General setup → Rust support, Kernel hacking → Sample kernel code → Rust samples
4. 빌드 → `insmod samples/rust/rust_minimal.ko` → `dmesg | tail` 에 `rust_minimal:` 접두 로그 → `rmmod`. 정확한 문구는 `samples/rust/rust_minimal.rs` 의 `pr_info!` 인자에서 인용할 것 — 기억으로 쓰지 않는다
5. 그 다음 `rust_minimal.rs` 를 읽으며 `module!` 매크로, `impl kernel::Module`, `pr_info!`, `Result`/`Error`(errno 매핑), `KBox`/`KVec`(할당 실패가 `Result`), `Arc`(커널 refcount), `Mutex`/`SpinLock`(lockdep 통합)
6. out-of-tree 모듈은 커널 트리 밖 `Kbuild` 로 빌드 가능하나 API 가 트리에 묶임
- 문서: `Documentation/rust/`(quick-start, coding-guidelines, general-information), https://rust-for-linux.com/, LWN 기사, Rust for Linux Zulip.

### 개념 매핑 (C 드라이버 → Rust)
| C | Rust |
|---|---|
| `module_init/exit` | `module!{}` + `impl Module { fn init() -> Result<Self> }` + `Drop` |
| `printk(KERN_INFO)` | `pr_info!` |
| `kmalloc` + NULL 체크 | `KBox::new(x, GFP_KERNEL)?` |
| `struct kref` | `Arc<T>` |
| `spin_lock_irqsave` | `SpinLock<T>` (가드, RAII) |
| `container_of` | 타입 안전 추상화 또는 `pin-init` 프로젝션 |
| 콜백 함수 포인터 테이블 | 트레이트 + vtable 생성 매크로 |

### 함정
- 커널은 `alloc` 의 `Box/Vec` 를 그대로 쓰지 않음(할당 실패가 패닉이 되기 때문) → `KBox`, `KVec`, `GFP_*` 플래그.
- 패닉 = 커널 oops. `unwrap()` 금지 문화. `#[pin_data]`/`pin_init!` 없이 `Mutex` 를 스택에 만들면 컴파일 에러.
- bindgen 버전, LLVM 버전, Rust 버전 조합이 빌드 실패의 90%. `Documentation/rust/quick-start.rst` 의 버전 표 준수.

## B. eBPF — aya
### 상태
- aya: 순수 Rust(libbpf 비의존), eBPF 쪽(`aya-ebpf`, `no_std`)과 유저스페이스(`aya`) 분리. 2026 FOSDEM: 스토리지 맵, 새 프로그램 타입, Rust eBPF 의 BTF 생성, **eBPF 타깃 Tier 2 승격 작업 중**(→ nightly 필요 여부는 확인 시점에 따라 다름).
- 대안: libbpf-rs(libbpf 바인딩, CO-RE 성숙), redbpf(사실상 중단).

### 첫 파이프라인
1. `cargo generate https://github.com/aya-rs/aya-template` (사용자가 실행) → 프로그램 타입 선택(xdp/kprobe/tracepoint/…)
2. 구조 이해: `<name>-ebpf/`(커널에서 실행, `#![no_std]`, 루프·스택 512B·헬퍼 제약), `<name>/`(로더, 맵 읽기), `<name>-common/`(공유 구조체 `#[repr(C)]`)
3. `cargo xtask run` 또는 `RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"'` (버전별 명령 확인)
4. 첫 과제: XDP 로 인터페이스 패킷 수 카운트(`PerCpuArray` 맵) 또는 `openat` tracepoint 에서 파일명 로그(`aya-log`)
- **검증기(verifier) 에러 읽기**가 핵심 스킬: 경계 검사 누락("R1 unbounded"), 루프, 스택 초과, 헬퍼 인자 타입. `bpftool prog dump xlated` 로 확인.

### 개념
- 맵 종류(HashMap, PerCpuArray, RingBuf, PerfEventArray, LRU, sk_storage…), CO-RE 와 BTF(`vmlinux.h` 없이 구조체 접근), 프로그램 타입별 컨텍스트, tail call, kfunc.
- 관측 도구로서의 eBPF: bpftrace 로 먼저 원샷 확인 → 같은 것을 aya 로.

## C. OS 직접 만들기
### 자료
- **Philipp Oppermann, Writing an OS in Rust** https://os.phil-opp.com/ — 사실상 표준 커리큘럼. 2판(edition 3) 진행 중: 1판 코드와 `bootloader` crate 버전(0.9 vs 0.11+) API 가 다르므로 어느 판을 따르는지 처음에 정한다.
- Redox OS(마이크로커널, 실제 사용 가능), Theseus, Hermit(유니커널), Tock(임베디드 OS, `rust-embedded-tutor` 연결), rust-vmm/firecracker(하이퍼바이저), rCore/xv6-riscv 의 Rust 포팅(교육용, 칭화대 rCore-Tutorial).
- OS 이론 병행: OSTEP 해당 장을 각 단계 전에 읽기.

### 단계 (blog_os 순서)
1. 프리스탠딩 바이너리: `#![no_std] #![no_main]`, `panic_handler`, 링커 인자, 커스텀 타깃 JSON(`x86_64-blog_os.json`) + `build-std` — **1.95 에서 stable 의 커스텀 타깃 지정 지원이 제거됨 → nightly 필요**(확인)
2. 부트: `bootloader` crate(BIOS/UEFI), QEMU 실행(`qemu-system-x86_64 -drive format=raw,file=...`), `bootimage`(1판) vs 빌드 스크립트(2판)
3. 출력: VGA 텍스트 버퍼(`volatile` 쓰기, 왜 volatile 인가), 시리얼(`uart_16550`) → 테스트 프레임워크(`custom_test_frameworks`, QEMU exit device)
4. CPU 예외: IDT(`x86_64` crate), 브레이크포인트, 더블 폴트 + IST 스택 전환(스택 오버플로 처리)
5. 하드웨어 인터럽트: PIC(`pic8259`) → 타이머, 키보드(`pc-keyboard`), 데드락(`without_interrupts`), 인터럽트 컨텍스트에서 할당 금지
6. 페이징: 4단계 페이지 테이블 직접 순회, 물리 메모리 매핑, 프레임 할당자(부트로더 메모리 맵), `OffsetPageTable`
7. 힙: `GlobalAlloc` 구현(bump → linked list → 고정 크기 블록), `alloc::Box/Vec` 사용 가능
8. 멀티태스킹: 협력적(async/await 실행기 — `Waker`, 키보드 스캔코드 스트림) → 선점형(컨텍스트 스위치, 스케줄러)은 커뮤니티 확장
9. 이후: 유저 모드/시스템 콜, 파일시스템, 네트워크(`smoltcp`), 다른 아키텍처(aarch64 Raspberry Pi 베어메탈 — `rust-raspberrypi-OS-tutorials`)

### 함정
- nightly 기능 의존(`build-std`, `custom_test_frameworks`, `abi_x86_interrupt`) → `rust-toolchain.toml` 로 nightly 고정(사용자가 작성).
- `bootloader` 0.9 → 0.11 API 대변화. 블로그 댓글/이슈에서 해당 버전 확인.
- 디버깅: `qemu -s -S` + `gdb`/`lldb`, `-d int` 로 인터럽트 로그, `bochs`. 시리얼 출력 없이 시작하면 좌절.

## 참고
- Rust for Linux: https://rust-for-linux.com/ · `Documentation/rust/` · LWN "Rust for Linux" 태그
- aya: https://aya-rs.dev/ · https://docs.rs/aya-ebpf · libbpf-rs
- blog_os: https://os.phil-opp.com/ · OSDev wiki https://wiki.osdev.org/ · rCore-Tutorial(중/영) · Redox https://www.redox-os.org/
- Intel SDM / AMD APM(x86 세부), ARM ARM(aarch64)
