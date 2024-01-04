# Use Docker Wasm image for node js apps

Build the Wasm container image for the node.js server. The total size of the image is less than 2MB.

```bash
docker buildx build --platform wasi/wasm -t secondstate/node-example-server .
```

Publish the Wasm container image to Docker Hub.

```bash
docker push secondstate/node-example-server
```

Run the Wasm container app.

```bash
docker run -dp 8080:8080 --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/node-example-server:latest
```

From another terminal, test the server application.

```bash
$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```
