# sway-easyfocus

A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.

https://github.com/edzdez/sway-easyfocus/assets/31393290/b71b1875-6c8b-4891-803c-763b040a55ff

## Usage

```
A tool to help efficiently focus windows in Sway inspired by i3-easyfocus

Usage: sway-easyfocus [OPTIONS]

Options:
      --window-background-color <WINDOW_BACKGROUND_COLOR>
          set the window background color <rrggbb> [default: 1d1f21]
      --window-background-opacity <WINDOW_BACKGROUND_OPACITY>
          set the window background opacity <0-1.0> [default: 0.2]
      --label-background-color <LABEL_BACKGROUND_COLOR>
          set the label background color <rrggbb> [default: 1d1f21]
      --label-background-opacity <LABEL_BACKGROUND_OPACITY>
          set the label background opacity <0-1.0> [default: 1]
      --font-family <FONT_FAMILY>
          [default: monospace]
      --font-weight <FONT_WEIGHT>
          [default: bold]
      --label-padding-x <LABEL_PADDING_X>
          [default: 4]
      --label-padding-y <LABEL_PADDING_Y>
          [default: 0]
      --label-margin-x <LABEL_MARGIN_X>
          [default: 4]
      --label-margin-y <LABEL_MARGIN_Y>
          [default: 2]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Build

This program is written in [Rust](https://www.rust-lang.org/). The Rust compiler can be installed by following the
instructions on the [official download page](https://www.rust-lang.org/tools/install).

You also need to have `gtk` and `gtk-layer-shell` installed.

```shell
# Clone this repo
$ git clone https://github.com/edzdez/sway-easyfocus.git

# Build with cargo
$ cargo build --release
$ ./target/release/sway-easyfocus

# Alternatively, build and run in one step
$ cargo run --release
```
