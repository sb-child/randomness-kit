from pathlib import Path
import subprocess
import sys

SCRIPT_DIR = Path(__file__).resolve().parent


def gen_ci() -> str:
    try:
        p = subprocess.run(
            ["maturin", "generate-ci", "github"],
            capture_output=True,
            check=True,
            text=True,
            cwd=SCRIPT_DIR,
        )
        ci_file: str = p.stdout
        return ci_file
    except FileNotFoundError:
        sys.exit("Error: maturin is not installed.")
    except subprocess.CalledProcessError as e:
        sys.exit(f"Generate CI failed: {e}")


def write_ci(s: str):
    ci_path = SCRIPT_DIR / ".github" / "workflows"
    ci_path.mkdir(parents=True, exist_ok=True)
    ci_file_path = ci_path / "CI.yml"
    with open(ci_file_path, "w") as f:
        f.write(s)


def main():
    print("Generating...")
    s = gen_ci()
    print("Saving CI file...")
    write_ci(s)
    print("Completed")


if __name__ == "__main__":
    main()
