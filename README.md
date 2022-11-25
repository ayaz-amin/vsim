# vsim

Vehicle simulation tool. Can mix and match different parts and go vroom vroom skrtt.

## WASM Usage

### one time

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install basic-http-server
```

### (re)build

```bash
make build-wasm
```

### running

```bash
# this only needs to be run once, and can be left running
basic-http-server public
```

or

```bash
make run-wasm
```

visit `http://127.0.0.1:4000` after to see simulator
