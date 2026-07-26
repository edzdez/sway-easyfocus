use clap::{ArgAction, Parser, Subcommand};

use serde::Deserialize;

use crate::util::Rgb;

/// What to do with the selected container.
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Command {
    /// Focus the container
    #[command(about = "Focus the selected window (default)")]
    Focus,

    /// Swap current window with selected window
    #[command(about = "Swap focused window with the selected window")]
    Swap {
        /// Also focus the selected window after swapping
        #[arg(long)]
        focus: bool,
    },

    /// Print the container's ID
    #[command(about = "Print the selected window's ID")]
    Print,
}

/// A tool to help efficiently focus windows in Sway, inspired by i3-easyfocus.
#[derive(Deserialize, Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The list of chars to use for hints
    #[arg(long)]
    pub chars: Option<String>,

    /// The window background color <rrggbb>
    #[arg(long)]
    pub window_background_color: Option<Rgb>,

    /// The window background opacity <0-1.0>
    #[arg(long)]
    pub window_background_opacity: Option<f64>,

    /// The label background color <rrggbb>
    #[arg(long)]
    pub label_background_color: Option<Rgb>,

    /// The label background opacity <0-1.0>
    #[arg(long)]
    pub label_background_opacity: Option<f64>,

    /// The label text color <rrggbb>
    #[arg(long)]
    pub label_text_color: Option<Rgb>,

    /// The label background color <rrggbb>
    #[arg(long)]
    pub focused_background_color: Option<Rgb>,

    /// The focused background opacity <0-1.0>
    #[arg(long)]
    pub focused_background_opacity: Option<f64>,

    /// The focused text color <rrggbb>
    #[arg(long)]
    pub focused_text_color: Option<Rgb>,

    /// The font family
    #[arg(long)]
    pub font_family: Option<String>,

    /// The font weight
    #[arg(long)]
    pub font_weight: Option<String>,

    /// The font size, see: https://www.w3.org/TR/css-fonts-3/#font-size-prop
    #[arg(long)]
    pub font_size: Option<String>,

    /// The label padding-x <px>
    #[arg(long)]
    pub label_padding_x: Option<i32>,

    /// The label padding-y <px>
    #[arg(long)]
    pub label_padding_y: Option<i32>,

    /// The label margin-x <px>
    #[arg(long)]
    pub label_margin_x: Option<i32>,

    /// The label margin-y <px>
    #[arg(long)]
    pub label_margin_y: Option<i32>,

    /// Show confirmation window after selection
    #[arg(
        long = "show-confirmation",
        action = ArgAction::Set,
        default_missing_value = "true",
        num_args = 0..=1
    )]
    pub show_confirmation: Option<bool>,

    /// The selected command
    #[command(subcommand)]
    #[serde(skip)]
    pub command: Option<Command>,
}
