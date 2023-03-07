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
    cargo build --release
    cp target/release/pastebin-server "$BACKEND"
    cp pastebin-server.toml "$BACKEND"
popd

pushd "$DIST"
    zip -r pastebin.dist."$TIME".zip frontend backend
    rm -rf frontend backend
popd

echo "done"
