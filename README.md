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
