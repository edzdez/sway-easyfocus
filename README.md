# sway-easyfocus

A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.

https://github.com/edzdez/sway-easyfocus/assets/31393290/b71b1875-6c8b-4891-803c-763b040a55ff

## Config File

The config file is found at `$XDG_CONFIG_HOME/sway-easyfocus/config.yaml`
If a key is not found in the config file nor provided as a cli option, then it uses a default value.

An example config is with the default options is shown below:

```yaml
window_background_color: 1d1f21
window_background_opacity: 0.2

label_background_color: 1d1f21
label_background_opacity: 1.0
label_text_color: c5c8c6

font_family: monospace
font_weight: bold
font_size: medium

label_padding_x: 4
label_padding_y: 0
label_margin_x: 4
label_margin_y: 2
```

## Usage

```
A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.

Usage: sway-easyfocus [OPTIONS]

Options:
      --window-background-color <WINDOW_BACKGROUND_COLOR>
          set the window background color <rrggbb>
      --window-background-opacity <WINDOW_BACKGROUND_OPACITY>
          set the window background opacity <0-1.0>
      --label-background-color <LABEL_BACKGROUND_COLOR>
          set the label background color <rrggbb>
      --label-background-opacity <LABEL_BACKGROUND_OPACITY>
          set the label background opacity <0-1.0>
      --label-text-color <LABEL_TEXT_COLOR>
          set the label text color <rrggbb>
      --font-family <FONT_FAMILY>

      --font-weight <FONT_WEIGHT>

      --font-size <FONT_SIZE>

      --label-padding-x <LABEL_PADDING_X>

      --label-padding-y <LABEL_PADDING_Y>

      --label-margin-x <LABEL_MARGIN_X>

      --label-margin-y <LABEL_MARGIN_Y>

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
