# LCV (*Line Coding Visualizer*)

![MSRV](https://img.shields.io/badge/MSRV-1.88.0-red?style=flat)

**LCV** is a **TUI-based** (Text User Interface) playground designed to help users explore and understand **line coding**
techniques used in digital communication systems.

<p align="center"><img src="./assets/demo.gif" width="550"/></p>

Its main goal is to visually represent how binary data is transformed into electrical waveforms, allowing users
to see in real time how different encoding schemes such as **NRZ, RZ, AMI, Manchester**, and others affect the signal.

## Features

With a lightweight and intuitive interface, LCV allows you to:

- Enter custom bit sequences.
- Dynamically switch between multiple line coding methods.
- Visualize how the waveform changes depending on the selected encoding scheme.

Perfect for **students, educators, and networking/telecommunications enthusiasts** who want a simple yet powerful
tool to **learn by visualizing**.

> **Note:** All line coding methods available in this playground are **bipolar schemes**, where the signal can switch
between positive, zero, and negative voltage levels depending on the encoding rules.

## Installation

## Building from source

### Requirements

- `rustup` / Cargo (Rust toolchain). Install from https://rustup.rs if needed.

### Steps

1. Clone this repository.
2. From the project root:
   - Debug build: `cargo build`
   - Release build (optimized): `cargo build --release`

> The final binary will be located in the `target` directory.

You can optionally install the binary system-wide or user-wide by using:
```
cargo install --path {path_to_project}
```

## Using a pre-built binary

If you prefer not to build from source, you can download a pre-built binary from the **Releases** section of this repository.

