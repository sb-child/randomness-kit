#!/usr/bin/env python3
import argparse
from pathlib import Path
import subprocess
import sys

SCRIPT_DIR = Path(__file__).resolve().parent


def generate_stub(env: None | dict = None):
    command = ["maturin", "generate-stubs", "-o", SCRIPT_DIR / "randomness_kit"]
    try:
        subprocess.run(command, cwd=SCRIPT_DIR, check=True, env=env)
    except KeyboardInterrupt:
        sys.exit(130)
    except FileNotFoundError:
        print(
            "Error: maturin is not installed.",
            file=sys.stderr,
        )
        sys.exit(1)


def generate_init_file(env: None | dict = None):
    command = ["uv", "run", "gen-init.py"]
    try:
        subprocess.run(command, cwd=SCRIPT_DIR, check=True, env=env)
    except KeyboardInterrupt:
        sys.exit(130)
    except FileNotFoundError:
        print(
            "Error: maturin is not installed.",
            file=sys.stderr,
        )
        sys.exit(1)


def develop(cmd):
    try:
        subprocess.run(cmd, cwd=SCRIPT_DIR, check=True)
    except KeyboardInterrupt:
        sys.exit(130)
    except FileNotFoundError:
        print(
            "Error: maturin is not installed.",
            file=sys.stderr,
        )
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(description="`maturin develop` 的快捷封装脚本")
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "-d",
        "--debug",
        action="store_true",
        help="以 Debug 模式运行（默认行为）",
    )
    group.add_argument(
        "-r",
        "--release",
        action="store_true",
        help="以 Release 模式运行 (--release)",
    )
    args, extra_args = parser.parse_known_args()
    cmd = ["maturin", "develop", "--uv", "--generate-stubs"]
    if args.release:
        cmd.append("--release")
    cmd.extend(extra_args)
    print("Generating Stubs...\n")
    develop(cmd)
    generate_stub()
    generate_init_file()


if __name__ == "__main__":
    main()
