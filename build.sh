
cd textglue-lib
cargo build
cd ..
cd textglue-wasm
wasm-pack build --target no-modules
cd ..
cd textglue-app
npm run build
cd ..
cd textglue-server
cargo build
cd ..
