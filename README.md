# vsim
Vehicle simulation tool. Can mix and match different parts and go vroom vroom skrtt.
## Usage

### one time
```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install basic-http-server
```

### (re)build
```bash
cargo build --target wasm32-unknown-unknown --release
```

> copy [./target/wasm32-unknown-unknown/release/vsim.wasm] into [./public]

```bash
basic-http-server public
```

visit `http://127.0.0.1:4000` after to see simulator 