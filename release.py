#!/usr/bin/env python3
import sys
import subprocess
import tomllib
from pathlib import Path

script_dir = Path(__file__).resolve().parent


def get_package_version() -> str:
    cargo_file = script_dir / "Cargo.toml"
    if not cargo_file.exists():
        print("Error: Could not found Cargo.toml")
        sys.exit(1)
    with open(cargo_file, "rb") as f:
        data = tomllib.load(f)
        version = data.get("package", {}).get("version")
        if version:
            return version
        else:
            print("Error: Could not get version info from Cargo.toml")
    sys.exit(1)


def tag_exists(tag_name: str) -> bool:
    result = subprocess.run(
        ["git", "tag", "-l", tag_name],
        capture_output=True,
        text=True,
        check=True,
        cwd=script_dir,
    )
    return bool(result.stdout.strip())


def main():
    version = get_package_version()
    tag = f"v{version}"
    print(f"Current Version: {version}")
    if tag_exists(tag):
        print(f"Error: Tag '{tag}' exists.")
        sys.exit(1)
    print("\nAdding files...")
    commit_res = subprocess.run(["git", "add", "."], cwd=script_dir)
    if commit_res.returncode != 0:
        print("Error: Add files failed.")
        sys.exit(1)
    commit_msg = f"Release {tag}"
    print("\nCommiting...")
    commit_res = subprocess.run(["git", "commit", "-am", commit_msg], cwd=script_dir)
    if commit_res.returncode != 0:
        print("Error: Commit failed.")
        sys.exit(1)
    target_tag = tag
    print("\nTagging...")
    tag_res = subprocess.run(
        ["git", "tag", "-a", target_tag, "-m", f"Version {version}"], cwd=script_dir
    )
    if tag_res.returncode != 0:
        print("Error: Tag failed.")
        sys.exit(1)
    print("Release completed.")


if __name__ == "__main__":
    main()
