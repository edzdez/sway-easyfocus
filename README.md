# sway-easyfocus

A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.

https://github.com/edzdez/sway-easyfocus/assets/31393290/b71b1875-6c8b-4891-803c-763b040a55ff

## Config File

The config file is found at `$XDG_CONFIG_HOME/sway-easyfocus/config.yaml`
If a key is not found in the config file nor provided as a cli option, then it uses a default value.

An example config is with the default options is shown below:

```yaml
chars: 'fjghdkslaemuvitywoqpcbnxz'

window_background_color: '1d1f21'
window_background_opacity: 0.2

label_background_color: '1d1f21'
label_background_opacity: 1.0
label_text_color: 'c5c8c6'

focused_background_color: '285577'
focused_background_opacity: 1.0
focused_text_color: 'ffffff'

font_family: monospace
font_weight: bold
font_size: medium

label_padding_x: 4
label_padding_y: 0
label_margin_x: 4
label_margin_y: 2

show_confirmation: true
```

## Usage

```
A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.

Usage: sway-easyfocus [OPTIONS] [COMMAND]

Commands:
  focus  Focus the selected window (default)
  swap   Swap focused window with the selected window
  print  Print the selected window's ID
  help   Print this message or the help of the given subcommand(s)

Options:
      --chars <CHARS>
          list of chars to use for hints <fjghdkslaemuvitywoqpcbnxz>
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
      --focused-background-color <FOCUSED_BACKGROUND_COLOR>
          set the label background color <rrggbb>
      --focused-background-opacity <FOCUSED_BACKGROUND_OPACITY>
          set the focused background opacity <0-1.0>
      --focused-text-color <FOCUSED_TEXT_COLOR>
          set the focused text color <rrggbb>
      --font-family <FONT_FAMILY>
          set the font family
      --font-weight <FONT_WEIGHT>
          set the font weight
      --font-size <FONT_SIZE>
          set the font size, see: https://www.w3.org/TR/css-fonts-3/#font-size-prop
      --label-padding-x <LABEL_PADDING_X>
          set the label padding-x <px>
      --label-padding-y <LABEL_PADDING_Y>
          set the label padding-y <px>
      --label-margin-x <LABEL_MARGIN_X>
          set the label margin-x <px>
      --label-margin-y <LABEL_MARGIN_Y>
          set the label margin-y <px>
      --show-confirmation <SHOW_CONFIRMATION>
          Show confirmation window after selection [possible values: true, false]
  -h, --help
          Print help
  -V, --version
          Print version
```

The default action is to focus the selected window.  The `swap`
command can be used to swap the focused window with the selected
window, and the `print` command can be used to print the selected
window ID (sway container ID).

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
