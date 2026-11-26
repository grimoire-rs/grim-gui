#!/usr/bin/env bash
# Run a command inside the grim-gui build container (webkit2gtk, xvfb, toolchain).
# Never takes the build lock: the caller owns it.
set -euo pipefail

if ! command -v docker >/dev/null 2>&1; then
    echo "docker not found" >&2
    exit 69
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dockerfile="$root/docker/linux-build.Dockerfile"
# The tag carries a hash of the Dockerfile, so an edit to it rebuilds the image.
image="grim-gui-build:$(sha256sum "$dockerfile" | cut -c1-12)"
link="$(cd "$root/.." && pwd)/.grimoire-split"

if ! docker image inspect "$image" >/dev/null 2>&1; then
    docker build -t "$image" -f "$dockerfile" "$root/docker"
fi

cache="$root/target/.container-cache"
mkdir -p "$cache/npm" "$cache/cargo" "$cache/uv"

args=(--rm --cap-drop ALL --security-opt no-new-privileges --user "$(id -u):$(id -g)"
-e HOME=/tmp
-e CARGO_HOME=/work-cache/cargo
-e npm_config_cache=/work-cache/npm
-e UV_CACHE_DIR=/work-cache/uv -e UV_PYTHON_DOWNLOADS=never -e UV_LINK_MODE=copy
-e GRIM_SOURCE_BINARY -e GRIM_BIN -e RUST_BACKTRACE
-v "$cache:/work-cache"
-v "$root:$root" -w "$root")

# The SDK path dependency lives behind the sibling link (see Cargo.toml).
if [ -e "$link" ]; then
    target="$(readlink -f "$link")"
    args+=(-v "$target:$target:ro" -v "$link:$link:ro")
else
    echo "in-container: $link is missing, the grim-sdk path dependency will not resolve" >&2
fi

exec docker run "${args[@]}" "$image" "$@"
