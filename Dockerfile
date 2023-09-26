FROM scratch
COPY ./spin.toml .
COPY ./frontend/index.html ./frontend/index.html
COPY ./target/wasm32-wasi/release/spin_static_fs.wasm ./target/wasm32-wasi/release/spin_static_fs.wasm
COPY ./target/wasm32-wasi/release/envvars.wasm ./target/wasm32-wasi/release/envvars.wasm
