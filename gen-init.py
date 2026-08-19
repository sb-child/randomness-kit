import subprocess

import randomness_kit
import tomllib
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent


def format_file(files: list[Path]):
    command = ["uv", "run", "ruff", "format", "--", *files]
    try:
        subprocess.run(command, cwd=SCRIPT_DIR, check=True)
    except FileNotFoundError as e:
        raise FileNotFoundError(f"ruff is not installed: {e}")


def get_package_version() -> str:
    cargo_file = SCRIPT_DIR / "Cargo.toml"
    if not cargo_file.is_file():
        raise FileNotFoundError("Cargo.toml not found.")
    try:
        data = tomllib.loads(cargo_file.read_bytes().decode("utf-8"))
        version = data.get("package", {}).get("version")
        if version:
            return version
    except Exception as e:
        raise ValueError(f"Error reading Cargo.toml: {e}")
    raise ValueError("Could not find version in Cargo.toml.")


def generate_init_file():
    mod_ver = randomness_kit.version()
    cargo_ver = get_package_version()
    assert mod_ver == cargo_ver, "Python module is out of date."
    assert randomness_kit.ping(666) == 666 * 2, "Python module is out of date."
    mod_all = randomness_kit._randomness_kit.__all__
    mod_all = mod_all if type(mod_all) is list else []
    mod_all_str = "*" if len(mod_all) == 0 else ", ".join(mod_all)
    mod_doc = str(randomness_kit.__doc__)
    save_path = SCRIPT_DIR / "randomness_kit"
    save_path.mkdir(parents=True, exist_ok=True)
    (save_path / "py.typed").touch(exist_ok=True)
    init_file = save_path / "__init__.py"
    template = f"""from ._randomness_kit import {mod_all_str}  # pyright: ignore[reportMissingImports] # noqa: F403
__all__ = {repr(mod_all)}  # pyright: ignore[reportUnsupportedDunderAll]  # noqa: F405
__doc__ = {repr(mod_doc)}
    """
    with open(init_file, "w") as f:
        f.write(template)
    format_file([init_file])


def main():
    print("Generating __init__.py ...")
    generate_init_file()
    print("Successfully generated.")


if __name__ == "__main__":
    main()
