FROM --platform=$BUILDPLATFORM rust:1.64 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        wget unzip
EOT
# This line installs WasmEdge including the AOT compiler
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM buildbase AS build
COPY server.js .
RUN wget https://github.com/second-state/wasmedge-quickjs/releases/download/v0.5.0-alpha/wasmedge_quickjs.wasm
RUN wget https://github.com/second-state/wasmedge-quickjs/releases/download/v0.5.0-alpha/modules.zip
RUN unzip modules.zip
# This line builds the AOT Wasm binary
RUN /root/.wasmedge/bin/wasmedgec wasmedge_quickjs.wasm wasmedge_quickjs.wasm

FROM scratch
ENTRYPOINT [ "wasmedge_quickjs.wasm", "server.js" ]
COPY --link --from=build /src/wasmedge_quickjs.wasm /wasmedge_quickjs.wasm
COPY --link --from=build /src/server.js /server.js
COPY --link --from=build /src/modules /modules
