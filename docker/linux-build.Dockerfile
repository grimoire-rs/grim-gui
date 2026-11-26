# Build and test environment for grim-gui on a host without webkit2gtk.
# Every download below is pinned to a version at least two weeks old and checked against a published digest.
FROM ubuntu:24.04@sha256:562111b88caa79c71391c0c4d3c2114a7d5c35915c20561fd893cb1e07929e6b
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
    libayatana-appindicator3-dev librsvg2-dev ca-certificates git pkg-config \
    xvfb xauth dbus dbus-x11 libgl1-mesa-dri python3 python3-venv xz-utils \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL -o /tmp/node.tar.xz https://nodejs.org/dist/v24.21.0/node-v24.21.0-linux-x64.tar.xz \
    && echo "fd8e59d5a511510f6a298afb548f18c7d2b1be404d8b4a27d94fbe49f56cb2d6  /tmp/node.tar.xz" | sha256sum -c - \
    && tar -xJf /tmp/node.tar.xz -C /usr/local --strip-components=1 --no-same-owner && rm /tmp/node.tar.xz
# World-readable toolchain so the run-time user (any uid) installs nothing.
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:/opt/uv:$PATH
RUN curl --proto '=https' --tlsv1.2 -fsSL -o /tmp/rustup-init https://static.rust-lang.org/rustup/archive/1.29.3/x86_64-unknown-linux-gnu/rustup-init \
    && echo "b1c8cb14e30889e7e2999e6b69e62d9e9b9ffc0f4df8180564d306d9680e6c55  /tmp/rustup-init" | sha256sum -c - \
    && chmod +x /tmp/rustup-init \
    && /tmp/rustup-init -y --profile minimal --default-toolchain 1.95.0 -c rustfmt -c clippy -c llvm-tools-preview \
    && rm /tmp/rustup-init \
    && cargo install --locked --version 0.9.150 cargo-nextest \
    && rm -rf /opt/cargo/registry /opt/cargo/git \
    && chmod -R a+rX /opt/rustup /opt/cargo
# Pinned prebuilt release; the release publishes no checksum file, so the digest was recorded at pin time.
RUN curl -fsSL -o /tmp/llvm-cov.tgz https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.9.1/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz \
    && echo "b3f68e625481fed9b16444174f3fa5ebcdbde4a1878803a35eabe2dcefcdc41a  /tmp/llvm-cov.tgz" | sha256sum -c - \
    && tar -xzf /tmp/llvm-cov.tgz -C /opt/cargo/bin cargo-llvm-cov && rm /tmp/llvm-cov.tgz \
    && chmod a+rx /opt/cargo/bin/cargo-llvm-cov
RUN curl -fsSL -o /tmp/uv.tgz https://github.com/astral-sh/uv/releases/download/0.13.6/uv-x86_64-unknown-linux-gnu.tar.gz \
    && echo "f29c3e94ffabd7e6f3ba521dba169a9b7ec12100bf3257087f0f24f6123f3645  /tmp/uv.tgz" | sha256sum -c - \
    && mkdir -p /opt/uv && tar -xzf /tmp/uv.tgz -C /opt/uv --strip-components=1 && rm /tmp/uv.tgz \
    && chmod -R a+rX /opt/uv
WORKDIR /work
