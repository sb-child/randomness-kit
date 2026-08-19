# Randomness Kit

some tools about randomness calculation

## How to use

todo

## Develop

Install maturin and ruff:

```bash
uv tool install maturin
uv tool install ruff
```

Build debug package, and install in local venv:

```bash
uv run dev.py
uv run python3
```

Build release package:

```bash
uv build
ls dist  # whl file generates here
```

Commit a release:

```bash
# Bump version at Cargo.toml
uv run dev.py
uv run release.py
```

Generate CI config:

```bash
uv run gen-ci.py
```
