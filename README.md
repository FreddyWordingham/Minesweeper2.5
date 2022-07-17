### Hi there 👋

#### Quickstart

Download the repository:

```shell
git clone https://github.com/FreddyWordingham/FreddyWordingham.git fw
cd fw
```

Compile the game:

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/main.wasm --out-dir ./app/static/wasm
```

Setup the webapp:

```shell
poetry install
```

Launch the server:

```shell
poetry run uvicorn app.api.main:app --reload
```