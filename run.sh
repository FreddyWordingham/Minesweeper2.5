# echo "Downloading repo..."
# git clone https://github.com/FreddyWordingham/FreddyWordingham.git fw
# cd fw

echo "Compiling game..."
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/main.wasm --out-dir ./app/static/wasm

echo "Copying assets..."
rm -r ./app/static/assets
cp -r ./assets ./app/static/

echo "Setup WebApp..."
poetry install

echo "Launch server..."
poetry run uvicorn app.api.main:app --reload
