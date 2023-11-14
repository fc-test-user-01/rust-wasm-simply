```
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  rai0kb/rust-wasm-simply \
  cp -r /workspace/dist /
```
