# YunoTools

## Build

```bash
cargo build --release
```

## Help

```bash
cargo run -p yunotools-cli -- --help
cargo run -p yunotools-cli -- --version
cargo run -p yunotools-cli -- -c --help
```

## Copyast

```bash
cargo run -p yunotools-cli -- -c
cargo run -p yunotools-cli -- -c . ./copyast-output.txt
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --dry-run
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --incremental
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --deduplicate
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --path-mode relative
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --token-model o200k
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --hidden
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --no-ignore
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --ignore-file .customignore
cargo run -p yunotools-cli -- -c . ./copyast-output.txt --max-file-size 10485760
```

## Ignore templates

```bash
cargo run -p yunotools-cli -- -c --list-templates
cargo run -p yunotools-cli -- -c --gen-ignore rust
cargo run -p yunotools-cli -- -c --gen-ignore rust,tauri -o ./.yunotools-ignore
```
