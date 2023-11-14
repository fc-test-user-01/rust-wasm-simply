```
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  rai0kb/rust-wasm-simply \
  cp -r /workspace/dist /
```

Or

```
docker build -t rust-wasm-simply .
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  rust-wasm-simply \
  cp -r /workspace/dist /
```

Or

```
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  -v $(pwd)/src/main.rs:/workspace/src/main.rs \
  -v $(pwd)/src/index.html:/workspace/dist/index.html \
  rai0kb/rust-wasm-simply \
  /bin/bash -c \
    "cargo build --manifest-path=/workspace/test-wasm/Cargo.toml --target wasm32-unknown-unknown && \
     wasm-bindgen /workspace/test-wasm/target/wasm32-unknown-unknown/debug/test-wasm.wasm --out-dir /workspace/dist && \
     cp -r /workspace/dist /"
```
