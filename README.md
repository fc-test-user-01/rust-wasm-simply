```
docker build -t rust:sandbox .
```

```
docker run -it --rm \
  -v $(pwd)/dist:/dist \
  --name rust-sandbox rust:sandbox \
  cp -r /workspace/dist /
```
