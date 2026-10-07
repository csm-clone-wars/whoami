# whoami
A reimplementation of the Linux command `whoami` in Rust.

## Overview
`whoami` displays the user name associated with the current effective user ID. Same as `id -un`.

Example:

```
whoami
```

```
csm
```

## Development Environment
- Operating System: Linux Mint
- Rust Toolchain

## Building the Project
- clone the repo
- cd into the cloned repo
- to build:

```
cargo build --release
```

- to build and run:

```
cargo run -- -help
```
## Commands

```
whoami <no args>     -> Print effective userid
whoami -version, -v  -> Print whoami version
whoami -help,    -h  -> Print whoami commands

```

## License
MIT