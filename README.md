# Randomness Kit

some tools about randomness calculation

## How to use

todo

## Develop

Install maturin:

```bash
uv tool install maturin
```

Build debug package, and install in local venv:

```bash
maturin develop --uv
uv run python3
```

Build release package:

```bash
uv build
ls dist  # whl file generates here
```

Commit a release:

```bash
uv run release.py
```

Generate CI config:

```bash
maturin generate-ci github > .github/workflows/CI.yml
```
