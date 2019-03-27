
cd textglue-lib
cargo build
cd ..
cd textglue-wasm
wasm-pack build --target no-modules
cd ..
cp textglue-wasm/pkg/textglue_wasm_bg.wasm textglue-app/public
cp textglue-wasm/pkg/textglue_wasm.js textglue-app/public
cd textglue-app
npm run build
cd ..
cd textglue-server
cargo build
cd ..
