FROM --platform=$BUILDPLATFORM python:3.12-slim-bullseye AS build
ENV HOME="/root"
WORKDIR $HOME

# Install build dependencies and set up Python environment
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    cmake \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    patchelf && \
    rm -rf /var/lib/apt/lists/* && \
    python -m venv $HOME/.venv && \
    .venv/bin/pip install --no-cache-dir "maturin>=1.7,<2.0"
ENV PATH="$HOME/.venv/bin:$PATH"

# Set Rust target based on the target platform
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/arm64") echo "aarch64-unknown-linux-musl" > rust_target.txt ;; \
  "linux/amd64") echo "x86_64-unknown-linux-gnu" > rust_target.txt ;; \
  *) exit 1 ;; \
  esac

# Install Rust toolchain
COPY rust-toolchain.toml rust-toolchain.toml
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --target $(cat rust_target.txt) --profile minimal --default-toolchain none && \
    . $HOME/.cargo/env && \
    rustup target add $(cat rust_target.txt)

# Copy project files and build
COPY kand kand
COPY kand-py kand-py
COPY Cargo.toml Cargo.lock pyproject.toml README.md LICENSE-MIT LICENSE-APACHE ./
COPY python python

# Build Python extension
RUN case "${TARGETPLATFORM}" in \
  "linux/arm64") \
    export JEMALLOC_SYS_WITH_LG_PAGE=16; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc; \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc; \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++; \
    ;; \
  *) \
    ;; \
  esac && \
  . $HOME/.cargo/env && \
  maturin build --release --target $(cat rust_target.txt) -i python3.12 && \
  mkdir -p /wheels && \
  cp target/wheels/*.whl /wheels/ && \
  rm -rf target $HOME/.cargo/registry $HOME/.cargo/git

# Final image: Provide Python runtime
FROM python:3.12-slim-bullseye
WORKDIR /app

# Copy and install the wheel file
COPY --from=build /wheels/ /wheels/
RUN pip install --no-cache-dir /wheels/*.whl && rm -rf \
    /wheels \
    /root/.cache \
    /usr/share/doc \
    /usr/share/man \
    /usr/share/locale \
    /var/lib/apt/lists/* \
    /var/cache/apt/archives/*

# Set Python path
ENV PYTHONPATH=/app
