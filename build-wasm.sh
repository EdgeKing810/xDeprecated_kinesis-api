#! /bin/bash

API_DIR="api"

# wget -qO- https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local
# sudo npm install -g wasmedge-core wasmedge-extensions

rm -rf pkg/
mkdir -p ${API_DIR}/node_modules/
rm -rf ${API_DIR}/node_modules/${NAME} ${API_DIR}/node_modules/.package-lock.json ${API_DIR}/package-lock.json target/wasm32*

rustwasmc clean
# wasm-pack build --debug
wasm-pack build --debug --target nodejs
# rustwasmc build --target deno
# rustwasmc build --target nodejs
# rustwasmc build

cd pkg/
# npm i wasmedge-core wasmedge-extensions
# cp ../wasm-modules-js.zip .
# unzip wasm-modules-js.zip
# rm wasm-modules-js.zip
# npm i wasmedge-core wasmedge-extensions
cd ..

cd ${API_DIR}/
npm i
