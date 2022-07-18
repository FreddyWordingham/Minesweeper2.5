#!/bin/bash

function download_repo() {
    echo "Downloading repo...";
    git clone https://github.com/FreddyWordingham/FreddyWordingham.git fw;
    cd fw;
}

function compile_wasm() {
    echo "Compiling wasm...";
    cargo build --release --target wasm32-unknown-unknown;
    wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/main.wasm --out-dir ./app/static/wasm --no-typescript;
}

function update_assets() {
    echo "Updating assets...";
    rm -r ./app/static/assets;
    rm -r ./docs/assets;
    cp -r ./assets ./app/static/;
    cp -r ./assets ./docs/assets/;
}

function update_build() {
    echo "Updating build...";
    rm -r ./docs/wasm;
    cp -r ./app/static/wasm ./docs;
}

function install_webapp() {
    echo "Installing WebApp...";
    poetry install;
}

function launch_server() {
    echo "Launching server...";
    poetry run uvicorn app.api.main:app --reload;
}

# download_repo
compile_wasm &
DEPS_PID=$!;
install_webapp
wait $DEPS_PID;
update_assets
update_build
launch_server