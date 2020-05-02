mkdir dist
mkdir dist/server

cd pastebin-front
export GENERATE_SOURCEMAP=false; yarn build
cd ..
cp -r pastebin-front/build dist/front

cd pastebin-server
cargo build --release
cd ..
cp pastebin-server/target/release/pastebin-server dist/server/
cp pastebin-server/pastebin-server.toml dist/server/
