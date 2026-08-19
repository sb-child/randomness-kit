#!/usr/bin/env python3
import argparse
import subprocess
import sys
import tomllib
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent


def run_git(*args: str, capture: bool = False) -> str:
    try:
        res = subprocess.run(
            ["git", *args],
            capture_output=capture,
            text=True,
            check=True,
            cwd=SCRIPT_DIR,
        )
        return res.stdout.strip() if capture else ""
    except FileNotFoundError:
        sys.exit("Error: Git is not installed.")
    except subprocess.CalledProcessError as e:
        sys.exit(f"Git command failed: {e}")


def get_package_version() -> str:
    cargo_file = SCRIPT_DIR / "Cargo.toml"
    if not cargo_file.is_file():
        sys.exit("Error: Cargo.toml not found.")
    try:
        data = tomllib.loads(cargo_file.read_bytes().decode("utf-8"))
        version = data.get("package", {}).get("version")
        if version:
            return version
    except Exception as e:
        sys.exit(f"Error reading Cargo.toml: {e}")
    sys.exit("Error: Could not find version in Cargo.toml.")


def main():
    parser = argparse.ArgumentParser(description="Release tool")
    parser.add_argument(
        "-m",
        "--message",
        type=str,
        metavar="MSG",
        required=False,
        default="",
        help="add some description in the commit message",
    )
    args = parser.parse_args()
    msg = str(args.message)
    msg = "" if len(msg) == 0 else f": {msg}"
    version = get_package_version()
    tag = f"v{version}"
    print(f"Current Version: {version}")
    if run_git("tag", "-l", tag, capture=True):
        sys.exit(f"Error: Tag '{tag}' already exists.")
    current_branch = run_git("branch", "--show-current", capture=True)
    if not current_branch:
        sys.exit("Error: Not on a valid branch (detached HEAD).")
    print()
    print("Adding files...")
    run_git("add", ".")
    print("Commiting...")
    run_git("commit", "--allow-empty", "-am", f"Release {tag}{msg}")
    print("Tagging...")
    run_git("tag", "-a", tag, "-m", f"Version {version}{msg}")
    print("Pushing branch...")
    run_git("push", "origin", current_branch)
    print("Pushing tag...")
    run_git("push", "origin", tag)
    print(f"\nSuccessfully released {tag} on '{current_branch}'.")


if __name__ == "__main__":
    main()
