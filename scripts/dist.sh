#!/bin/bash -ex

TIME=$(date -u +"%Y%m%d%H%M%S")

DIST="$PWD"/dist
FRONTEND="$DIST"/frontend
BACKEND="$DIST"/backend

mkdir -p "$FRONTEND"
mkdir -p "$BACKEND"

pushd pastebin-front
    npm install
    npm run build
    cp -r dist/* "$FRONTEND"
popd

pushd pastebin-server
    if [ -n "$ZIGBUILD" ]; then
        cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.27
        cp target/x86_64-unknown-linux-gnu/release/pastebin-server "$BACKEND"
    else
        cargo build --release
        cp target/release/pastebin-server "$BACKEND"
    fi
    cp pastebin-server.toml "$BACKEND"
popd

pushd "$DIST"
    zip -r pastebin.dist."$TIME".zip frontend backend
    rm -rf frontend backend
popd

echo "done"
