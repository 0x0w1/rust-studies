#!/usr/bin/env python3
"""PreToolUse guard: Rust 학습 저장소에서 Claude가 코드/빌드 설정을 직접 바꾸지 못하게 막는다.

- Edit/Write/NotebookEdit → 보호 대상 파일(*.rs, Cargo.toml, Cargo.lock, build.rs,
  rust-toolchain*, .cargo/config*) 쓰기를 차단
- Bash → cargo add/remove/new/init/update/install/fix, rustup 변경 명령,
  보호 대상 파일에 대한 리다이렉트·sed -i·tee·mv·cp·rm 을 차단

사용자가 명시적으로 수정을 허용하고 싶으면 잠금 해제 파일을 만든다:
    touch .claude/rust-edit-unlock      (해제)
    rm    .claude/rust-edit-unlock      (다시 잠금)
잠금 해제 파일은 사용자가 직접 만드는 것이 원칙이다. Claude가 스스로 만들면 안 된다.
"""
import json
import os
import re
import sys

PROTECTED_FILE = re.compile(
    r"(\.rs|Cargo\.toml|Cargo\.lock|build\.rs|rust-toolchain(\.toml)?|\.cargo/config(\.toml)?)$"
)

CARGO_MUTATING = re.compile(
    r"\bcargo\s+(\+\S+\s+)?(add|remove|rm|new|init|install|uninstall|update|fix|generate|publish|yank|vendor)\b"
)
CLIPPY_FIX = re.compile(r"\bcargo\s+(\+\S+\s+)?clippy\b.*--fix")
RUSTUP_MUTATING = re.compile(
    r"\brustup\s+(toolchain\s+(install|uninstall|link)|target\s+(add|remove)|component\s+(add|remove)|override|default|update|self\s+update|set)\b"
)
# 셸에서 보호 파일에 쓰는 흔한 패턴들
SHELL_WRITE = re.compile(
    r"(>{1,2}\s*\S*" + PROTECTED_FILE.pattern[:-1] + r")"          # > foo.rs, >> Cargo.toml
    r"|(\bsed\s+-i\b.*" + PROTECTED_FILE.pattern[:-1] + r")"        # sed -i ... foo.rs
    r"|(\btee\s+(-a\s+)?\S*" + PROTECTED_FILE.pattern[:-1] + r")"   # tee foo.rs
    r"|(\b(mv|cp|rm|truncate|touch)\s+.*" + PROTECTED_FILE.pattern[:-1] + r")"
)


def project_dir() -> str:
    return os.environ.get("CLAUDE_PROJECT_DIR") or os.getcwd()


SCRATCH_PREFIXES = ("/tmp/", "/private/tmp/", "/var/folders/", "/private/var/folders/")


def outside_project(path: str) -> bool:
    """저장소 밖(스크래치/임시 디렉터리)의 파일은 보호하지 않는다 — 확인용 스크래치 컴파일은 허용."""
    if not path.startswith("/"):
        return False
    if path.startswith(SCRATCH_PREFIXES):
        return True
    proj = os.path.realpath(project_dir())
    return not os.path.realpath(path).startswith(proj + os.sep)


def unlocked() -> bool:
    return os.path.exists(os.path.join(project_dir(), ".claude", "rust-edit-unlock"))


def deny(reason: str) -> None:
    sys.stderr.write(
        "[rust-edit-guard] 차단됨: " + reason + "\n"
        "이 저장소는 학습용이라 Claude는 가이드만 제공하고 Rust 코드·Cargo 설정은 사용자가 직접 수정합니다.\n"
        "사용자에게 무엇을 어떻게 바꾸면 되는지 설명하세요. 사용자가 정말로 Claude가 직접 수정하길 원한다면 "
        "사용자가 터미널에서 `! touch .claude/rust-edit-unlock` 를 실행해 잠금을 풀 수 있고, "
        "`! rm .claude/rust-edit-unlock` 로 다시 잠글 수 있다고 안내하세요. Claude가 이 파일을 대신 만들지 마세요.\n"
    )
    sys.exit(2)


def main() -> None:
    try:
        payload = json.load(sys.stdin)
    except Exception:
        sys.exit(0)

    if unlocked():
        sys.exit(0)

    tool = payload.get("tool_name", "")
    inp = payload.get("tool_input", {}) or {}

    if tool in ("Edit", "Write", "NotebookEdit"):
        path = inp.get("file_path") or inp.get("notebook_path") or ""
        if PROTECTED_FILE.search(path) and not outside_project(path):
            deny(f"{tool} → {path}")
        sys.exit(0)

    if tool == "Bash":
        cmd = inp.get("command", "")
        # 잠금 해제 파일을 Claude가 스스로 만드는 것도 막는다
        if re.search(r"rust-edit-unlock", cmd) and re.search(r"\b(touch|echo|printf|tee|cp|mv|>)\b|>", cmd):
            deny("잠금 해제 파일은 사용자가 직접 만들어야 합니다: " + cmd)
        if CARGO_MUTATING.search(cmd) or CLIPPY_FIX.search(cmd):
            deny("cargo 프로젝트/의존성 변경 명령: " + cmd)
        if RUSTUP_MUTATING.search(cmd):
            deny("rustup 툴체인 변경 명령: " + cmd)
        if SHELL_WRITE.search(cmd):
            # 스크래치 디렉터리에서 실행되는 셸 쓰기는 허용 (cd /tmp/... && ... > x.rs)
            if not (re.search(r"(^|[\s;&|])cd\s+(/private)?/(tmp|var/folders)/", cmd) or
                    re.search(r">{1,2}\s*(/private)?/(tmp|var/folders)/\S*", cmd)):
                deny("셸을 통한 보호 파일 쓰기: " + cmd)
        sys.exit(0)

    sys.exit(0)


if __name__ == "__main__":
    main()
