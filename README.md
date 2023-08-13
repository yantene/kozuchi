# Kozuchi

Kozuchi is a practice project for me.
It's a tool for compression and decompression.
You probably shouldn't expect much from it, though.
It's slow, and the compression ratio isn't great, but hey, it's a learning experience, right?

**Please do not use this for utilitarian purposes!**

## Build

The following command will create an executable `./target/release/kozuchi`.

```bash
cargo build --release
```

## Usage

### Compression

```
kozuchi compress <original file> [-o <compressed file>]
```

### Decompression

```
kozuchi decompress <compressed file> [-o <decompressed file>]
```

## Warnings

- **Speed**: It's slow, okay? Don't expect it to be fast.
- **Compression Ratio**: Don't expect much here either.
