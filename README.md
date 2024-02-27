# Wrm - Remove files or directories

![workflow_build](https://github.com/p1486/sdmw/actions/workflows/build.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/wrm)

## Installation
Run the following Cargo command:
```
cargo install wrm
```

## Usage
To move files to trash(`$HOME/.local/share/wrm/trash`):
```console
wrm foo bar ...
```

To destroy files:
```console
wrm --destroy ~/.local/share/wrm/trash/foo ~/.local/share/wrm/trash/bar ...
```

To restore files in trash to where they came from:
```console
wrm --restore ~/.local/share/wrm/trash/foo ~/.local/share/wrm/trash/bar ...
```

To list all files and directories in trash:
```console
wrm --list
```

To delete all files and directories in trash permanently:
```console
wrm --empty
```

### Options
```console
-n, --noninteractive  Do not prompt whether to change destinations
-q, --quiet           Do not print log messages
-h, --help            Print help
-V, --version         Print version
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## TODO
- Add shell completions
- Add auto empty
- Multiplatform support
- Optimize performance
