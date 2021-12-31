#! /bin/bash

API_DIR="api"

# wget -qO- https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local
# sudo npm install -g wasmedge-core wasmedge-extensions

rm -rf pkg/
mkdir -p ${API_DIR}/node_modules/
rm -rf ${API_DIR}/node_modules/${NAME} ${API_DIR}/node_modules/.package-lock.json ${API_DIR}/package-lock.json target/wasm32*

wasm-pack build --release --target nodejs

cd ${API_DIR}/
npm i
