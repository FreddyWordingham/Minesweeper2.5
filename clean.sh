#!/bin/bash

cargo clean;
rm -r ./app/static/assets;
rm -r ./app/static/wasm;
rm -r ./docs/wasm;
rm poetry.lock;
rm -r .venv;