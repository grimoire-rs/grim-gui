# Build and test environment for grim-gui on a host without webkit2gtk.
FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
    libayatana-appindicator3-dev librsvg2-dev ca-certificates git pkg-config \
    xvfb xauth dbus dbus-x11 libgl1-mesa-dri python3 python3-venv \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
# World-readable toolchain so the run-time user (any uid) installs nothing.
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:/opt/uv:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain 1.95.0 -c rustfmt -c clippy -c llvm-tools-preview \
    && cargo install --locked cargo-nextest \
    && rm -rf /opt/cargo/registry /opt/cargo/git \
    && chmod -R a+rX /opt/rustup /opt/cargo
# Pinned prebuilt release; the release publishes no checksum file, so the digest was recorded at pin time.
RUN curl -fsSL -o /tmp/llvm-cov.tgz https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.9.1/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz \
    && echo "b3f68e625481fed9b16444174f3fa5ebcdbde4a1878803a35eabe2dcefcdc41a  /tmp/llvm-cov.tgz" | sha256sum -c - \
    && tar -xzf /tmp/llvm-cov.tgz -C /opt/cargo/bin cargo-llvm-cov && rm /tmp/llvm-cov.tgz \
    && chmod a+rx /opt/cargo/bin/cargo-llvm-cov
RUN curl -LsSf https://astral.sh/uv/install.sh | env UV_INSTALL_DIR=/opt/uv UV_NO_MODIFY_PATH=1 sh \
    && chmod -R a+rX /opt/uv
WORKDIR /work
