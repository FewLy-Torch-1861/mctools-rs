# mctools-rs

A simple miscellaneous command-line tool for Minecraft, written in Rust.

## Features

- ✅ Convert Overworld coordinates to Nether coordinates.
- ✅ Convert Nether coordinates back to Overworld coordinates.

## Prerequisites

You need to have the Rust toolchain installed. If you don't have it, you can install it via [rustup](https://rustup.rs/).

## Installation

There are two ways to install mctools:

1. Download a pre-built binary from the latest release

   - Visit the releases page: https://github.com/FewLy-Torch-1861/mctools-rs/releases
   - Download the appropriate asset for your platform (for example: Linux, macOS, or Windows).
   - Extract the archive if necessary and move the `mctools` (or `mctools.exe` on Windows) binary to a directory on your PATH (for example `~/.local/bin` on Linux/macOS or `C:\Program Files\` on Windows).
   - Verify installation:
     ```bash
     mctools --help
     ```

2. Clone the repository and build from source

   - Clone the repository:
     ```bash
     git clone https://github.com/FewLy-Torch-1861/mctools-rs.git
     cd mctools-rs
     ```

   - Install the binary using Cargo (this installs to your Cargo bin path, typically `~/.cargo/bin`):
     ```bash
     cargo install --path .
     ```

   - Or build locally for testing without installing:
     ```bash
     cargo build --release
     # The optimized binary will be at target/release/mctools
     ```

   - After installing with Cargo, ensure your Cargo bin path is on your PATH:
     ```bash
     export PATH="$HOME/.cargo/bin:$PATH"
     ```

## Usage

This tool provides subcommands for different functionalities. (but currently only has one command)

### `nether-coord`

This command converts coordinates between the Overworld and the Nether. The coordinate ratio is 1:8 (Overworld:Nether).

**Syntax:**

```
mctools-rs nether-coord [OPTIONS] <X> <Z>
```

---

#### **Example 1: Convert Overworld to Nether**

To convert Overworld coordinates `X=1000`, `Z=-800` to Nether coordinates:

```bash
$ mctools-rs nether-coord 1000 -800
xz: 125, -100
```

---

#### **Example 2: Convert Nether to Overworld**

To convert Nether coordinates `X=125`, `Z=-100` back to Overworld coordinates, use the `--reverse` or `-r` flag.

**Using `--reverse`:**

```bash
$ mctools-rs nether-coord --reverse 125 -100
xz: 1000, -800
```

**Using `-r`:**

```bash
$ mctools-rs nether-coord -r 125 -100
xz: 1000, -800
```
