# Installation Guide

Get started with `kand` through Python, Rust, or Docker. This guide covers all installation methods and system compatibility details.

## Python Installation

### Requirements
- Python 3.8+
- `pip` (Python package installer)

### Install from PyPI
Install `kand` with one command—precompiled wheels available for instant setup:

```bash
pip install kand
```

!!! tip "Supported Platforms & Python Versions"
    We provide precompiled packages on PyPI for major systems and Python versions:

    | Platform     | Supported Python Versions         |
    |--------------|-----------------------------------|
    | **Linux**    | 3.8, 3.9, 3.10, 3.11, 3.12       |
    | **musl Linux** | 3.8, 3.9, 3.10, 3.11, 3.12     |
    | **Windows**  | 3.8, 3.9, 3.10, 3.11, 3.12, 3.13 |
    | **macOS**    | 3.8, 3.9, 3.10, 3.11, 3.12, 3.13 |

    No compilation needed—just `pip install` and go!

## Rust Installation

### Requirements
- Rust 1.80+
- `cargo` (Rust package manager)

### Add as a Dependency
Incorporate `kand` into your Rust project:

```toml
[dependencies]
kand = "0.1.0"  # Check latest version on crates.io
```

Or use the CLI:

```bash
cargo add kand
```

## Docker Usage

### Pull the Official Image
Grab the latest `kand` container:

```bash
docker pull ghcr.io/rust-ta/kand:latest
```

### Run with Docker
Launch it interactively:

```bash
docker run -it --rm ghcr.io/rust-ta/kand:latest
```

Or build your own:

```bash
docker build -t my-kand-app .
docker run -it --rm my-kand-app
```

## Troubleshooting

Encounter issues? Try these steps:

1. Update `pip` or `cargo` to the latest version.
2. Verify Python (3.8+) or Rust (1.80+) compatibility.
3. Ensure the Docker daemon is running.
4. Check [GitHub Issues](https://github.com/rust-ta/kand/issues) for solutions.

!!! note
    Still stuck? Join our community or file an issue on GitHub!

## Next Steps

- Explore the [API Documentation](api.md) to dive into `kand`.
- Join our community for help and updates.
- Report bugs or suggestions on [GitHub](https://github.com/rust-ta/kand/issues).
