# 유저스페이스 시스템 프로그래밍 — Rust 로 OS 를 실험하기 (2026-09 스냅샷)

## 세 층 구조
| 층 | crate | 특징 | 언제 |
|---|---|---|---|
| std | `std::fs/net/process/thread/os::unix` | 안전, 이식성, 대부분 충분 | 기본 |
| 안전 래퍼 | **rustix**(I/O safety, `OwnedFd`/`BorrowedFd`, 직접 syscall 백엔드 가능), **nix**(가장 넓은 *nix 커버리지) | 대부분 안전 API, 타입으로 fd 소유권 표현 | std 가 노출하지 않는 syscall (epoll, mmap 옵션, timerfd, signalfd, fork, pipe2, sendmsg/SCM_RIGHTS…) |
| raw | **libc** | C 시그니처 그대로, 거의 전부 unsafe | 래퍼에 없는 것, 래퍼가 어떻게 만들어지는지 배울 때 |

`std::os::fd::{OwnedFd, BorrowedFd, AsFd}` — I/O safety(1.63+): fd 를 정수가 아니라 소유권 있는 타입으로. "닫힌 fd 를 쓰는 버그"가 타입 에러로.

## 주제별 실험 로드맵
### 관찰 도구 (코드 전에)
- `strace -f -e trace=%file,%network ./bin`, `strace -c`(syscall 통계), `ltrace`(라이브러리 호출), `perf trace`, `bpftrace -e 'tracepoint:syscalls:sys_enter_openat { printf("%s\n", str(args->filename)); }'`
- `/proc/<pid>/{maps,status,fd,stack}`, `pmap`, `lsof -p`, `ss -tlnp`
- macOS: `dtruss`(SIP 제약), `fs_usage`, `vmmap`, Instruments. 리눅스 VM(UTM/Lima/multipass) 권장.

### 파일과 fd
- `File::open` → `openat(2)`. `O_CLOEXEC` 가 기본인 이유(exec 시 fd 누출). `read` 가 요청보다 적게 읽는 것(short read) 처리. `fsync` vs `fdatasync`, 페이지 캐시, `O_DIRECT`.
- 실험: 1GB 파일을 4KB/64KB/1MB 버퍼로 읽고 `strace -c` 로 syscall 수 비교 → `BufReader` 의 존재 이유.
- `mmap`(rustix `mm::mmap` / `memmap2` crate): 파일을 메모리로. 왜 `&[u8]` 로 노출하는 것이 unsound 한가(다른 프로세스가 파일을 바꿀 수 있음) → `memmap2` 의 unsafe 표시.

### 프로세스
- `Command`(std) → `fork`+`exec`(`nix::unistd::fork`, `execvp`). fork 후 자식에서 할당 금지(async-signal-safe), `posix_spawn`/`vfork`/`clone3` 비교.
- 좀비와 `waitpid`, 프로세스 그룹, `setsid`, 데몬화. `pipe2` + `dup2` 로 셸 파이프라인(`ls | grep`) 구현 — 중급 최고 과제.
- 시그널: `signal-hook`(안전) / `ctrlc` / `nix::sys::signal`. 핸들러에서 할 수 있는 일의 제약, `signalfd` 로 시그널을 fd 로 바꾸기, `SA_RESTART` 와 `EINTR`.

### 스레드와 동기화 (OS 관점)
- `std::thread` → `clone(2)` (`strace -f` 로 확인). futex 가 `Mutex` 의 기반(`strace` 에서 `futex(...)` 관찰). 스핀락 vs 뮤텍스 비용 측정. `parking_lot`. 스레드 어피니티(`core_affinity`), NUMA.
- `thread::scope`, `Arc<Mutex>` vs 채널, 원자적 카운터 → `../../rust-tutor/references/advanced-rust.md`의 메모리 모델.

### 네트워크와 I/O 멀티플렉싱 — 비동기의 뿌리
1. 블로킹 TCP 에코 서버(std) → 클라이언트 2개로 한계 관찰
2. 스레드-퍼-커넥션 → 1만 연결 실험(`ulimit -n`)
3. 논블로킹 + `epoll`(rustix `event::epoll`) 직접 — readiness 모델. `EPOLLET` vs 레벨 트리거, `EAGAIN` 처리
4. `mio` 소스 읽기 → tokio 의 리액터가 이것
5. `io_uring`(`io-uring` crate) — completion 모델, SQ/CQ 링, 커널 5.x+ 기능 차이. tokio-uring/monoio/glommio
- 소켓 옵션(`SO_REUSEADDR/PORT`, `TCP_NODELAY`), `sendfile`/`splice` 제로카피, Unix 도메인 소켓과 `SCM_RIGHTS` 로 fd 전달, `SOCK_CLOEXEC`.

### 메모리
- 가상 메모리: `/proc/self/maps` 를 Rust 프로그램 안에서 출력해 스택/힙/mmap/공유 라이브러리 확인. `brk` vs `mmap` 할당 임계값(glibc `M_MMAP_THRESHOLD`).
- `GlobalAlloc` 으로 커스텀 할당자(bump → free-list → buddy/slab), `#[global_allocator]`. jemalloc/mimalloc 교체 후 벤치.
- 페이지 폴트 관찰(`perf stat -e page-faults`), `madvise`, huge pages, `mlock`.
- 스택: 스레드 스택 크기(`Builder::stack_size`), 스택 오버플로 감지(guard page → SIGSEGV), `RUST_MIN_STACK`.

### 컨테이너 내부 (OS 격리 기능)
- namespaces(`unshare`, `clone` 플래그 `CLONE_NEWPID/NEWNS/NEWNET`), `pivot_root`, cgroups v2(`/sys/fs/cgroup`), seccomp(`seccompiler`, `libseccomp`), capabilities(`caps`).
- 프로젝트: 200줄짜리 미니 컨테이너 런타임(`youki` 가 Rust 로 된 완전한 OCI 런타임 — 소스 참고). **반드시 VM 안에서.**

### 파일시스템
- FUSE(`fuser` crate) 로 유저스페이스 파일시스템 — inode/dentry/VFS 개념을 안전하게 실험. 메모리 FS → 암호화 FS.
- ext4 이미지 파서(`no_std` 가능) — 온디스크 구조 읽기.

### FFI 와 ABI
- `bindgen`(C 헤더→Rust), `cbindgen`(Rust→C), `#[repr(C)]`, 호출 규약, `extern "C-unwind"`. 동적 로딩(`libloading`, `dlopen`). ELF 구조(`goblin`, `readelf`, `objdump`), 심볼·재배치·PLT/GOT, 링커 스크립트, `RPATH`. 정적/동적 링킹, musl vs glibc.

## 안전성 계약 체크리스트 (unsafe 래퍼 만들 때)
- fd: 소유 vs 빌림(`OwnedFd` vs `BorrowedFd`), 두 번 close 금지, exec 후 누출(CLOEXEC)
- 포인터/버퍼: 길이·정렬·수명, 커널이 쓰는 버퍼는 `MaybeUninit`, `read` 반환값만큼만 초기화됨
- 시그널 핸들러: async-signal-safe 함수만, `AtomicBool`/`write(2)` 로 통신
- fork: 자식에서 할당·락 금지
- mmap: 파일이 외부에서 바뀔 수 있으면 `&[u8]` 로 노출 불가
- 에러: `errno` 는 즉시 읽기(`io::Error::last_os_error()`), `EINTR` 재시도

## 참고
- *The Linux Programming Interface*(Kerrisk), *OSTEP*(무료 https://pages.cs.wisc.edu/~remzi/OSTEP/), *CS:APP*
- rustix https://docs.rs/rustix · nix https://docs.rs/nix · libc https://docs.rs/libc · I/O safety RFC 3128
- man pages(`man 2 openat`), https://man7.org/ · `std` 소스 `library/std/src/sys/pal/unix/`
- youki(컨테이너 런타임) https://github.com/youki-dev/youki · fuser https://docs.rs/fuser · io-uring https://docs.rs/io-uring
