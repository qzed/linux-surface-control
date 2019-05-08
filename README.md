# Linux Surface Control

Control various aspects of Microsoft Surface devices on Linux from the Command-Line.
Aims to provide a unified front-end to the various sysfs-attributes and special devices.

## Usage

```
USAGE:
    surface [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Keep output quiet
    -V, --version    Prints version information

SUBCOMMANDS:
    dgpu           Control or query the dGPU power state on the Surface Book 2
    help           Prints this message or the help of the given subcommand(s)
    latch          Control the latch/dtx-system on the Surface Book 2
    performance    Control or query the current performance-mode
    status         Query the current system status
```

See `surface <subcommand> help` for more details.

_Hint:_ You can specify the subcommand by any unabiguous prefix of it, i.e. `surface perf` and `surface p` will both evaluate to `surface performance`.

## Prequisites

For this tool to work, you need a recent version of the [surface-acpi module][surface-acpi] e.g. via [Jake's kernel][surface-kernel] (for `latch` and `performance` sub-commands) and the [dGPU hot plug module][surface-shps] (for the `dgpu` sub-command).

## Installing

Have a look at the [releases](https://github.com/qzed/linux-surface-control/releases) page.
Pre-built packages are available for Debian (Ubuntu, ...) and Arch Linux (Manjaro, ...) based distributions.

## Building from Source

Building this application from source follows the standard rust procedure, i.e. simply call `cargo build --release --locked` for a release-ready executable.
Completion files are automatically generated and can be found in the corresponding `target/release/build/surface-<hash>/out/` directory.

### Arch Linux

You can generate a package using the provided PKGBUILD in the `pkg/arch` directory and install it using `makepkg -si`.

### Debian-based Distributions (Ubuntu, ...)

Generating a Debian package can be done via [`cargo deb`](https://github.com/mmstick/cargo-deb).
Specifically, you need to run
```
env CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 cargo deb
```
where setting `CARGO_TARGET_DIR` is required to output the generated auto-completion files at the correct location for cargo-deb to pick up.

The final package can be found in `target/debian`.

[surface-acpi]: https://github.com/qzed/linux-surfacegen5-acpi
[surface-kernel]: https://github.com/jakeday/linux-surface
[surface-shps]: https://github.com/qzed/linux-surfacebook2-mshw0153
