set -e
set -x

rm -rf dist
mkdir dist
mkdir dist/frontend
mkdir dist/backend

pushd pastebin-front
GENERATE_SOURCEMAP=false npm run build
popd
cp -r pastebin-front/build/* dist/frontend

pushd pastebin-server
cargo build --release
popd
cp pastebin-server/target/release/pastebin-server dist/backend
cp pastebin-server/pastebin-server.toml dist/backend

TIME=`date -u +"%Y%m%d%H%M%S"`
pushd dist
zip -r pastebin.dist$TIME.zip frontend backend
popd

echo 'done'
