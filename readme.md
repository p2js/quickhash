# quickhash

Quickhash is a zero-dependency CLI applet to quickly get checksums of desktop files, built in rust.
The code currently features self-made implementations for MD5 and SHA256 checksums.
Because quickhash currently loads entire source files into memory before computing the checksums, its use is currently only recommended on small files.

## Building

With rust installed, a simple `cargo build --release` should produce the executable you need to get started.

## Usage

```quickhash sources [--hash h]```

- `quickhash` should be replaced with the path of the executable.
- `sources` should be a space-separated list of directories for the files that checksums should be computed for.
- `--hash h` is an optional flag, where h is the hashing algorithm you want to use (MD5/SHA256). Quickhash will output MD5 checksums by default.