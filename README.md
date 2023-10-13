# Flatr

### Prints JSON files in a flat format.
#### This is a rust rewrite of the [catj](https://github.com/soheilpro/catj) tool.

![Demo of usage](./docs/demo.gif)

## Usage
```bash
flatr --file my-file.json
```

## Build
```bash
git clone https://github.com/jfvillablanca/flatr
cd flatr
cargo build --release
./target/release/flatr --help
```

## Install
```bash
cargo install flatr
```

## Build with Nix
```bash
nix build github:jfvillablanca/flatr
```

## Run with Nix
```bash
nix run github:jfvillablanca/flatr -- -f my-file.json
```

## Motivation
I'm currently learning rust :).

