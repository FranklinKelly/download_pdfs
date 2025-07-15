# Download Pdfs Tool (Rust CLI)

Takes a csv file formatted like so:

| name | pdf url |
| ---- | ------- |
| pdf1 | https://example.com/media/pdf1 

And downloads them

## Use

```sh
cargo run <LINKS_PATH> <DOWNLOAD_PATH>
```

```sh
cargo run --help
```