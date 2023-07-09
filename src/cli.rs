use clap::Parser;

/// A tool to help efficiently focus windows in Sway inspired by i3-easyfocus.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// set the window background color <rrggbb>
    #[arg(long, default_value = "1d1f21")]
    pub window_background_color: String,

    /// set the window background opacity <0-1.0>
    #[arg(long, default_value_t = 0.2)]
    pub window_background_opacity: f64,

    /// set the label background color <rrggbb>
    #[arg(long, default_value = "1d1f21")]
    pub label_background_color: String,

    /// set the label background opacity <0-1.0>
    #[arg(long, default_value_t = 1.0)]
    pub label_background_opacity: f64,

    // set the font family
    #[arg(long, default_value = "monospace")]
    pub font_family: String,

    // set the font weight
    #[arg(long, default_value = "bold")]
    pub font_weight: String,

    // set the label padding-x <px>
    #[arg(long, default_value_t = 4)]
    pub label_padding_x: i32,

    // set the label padding-y <px>
    #[arg(long, default_value_t = 0)]
    pub label_padding_y: i32,

    // set the label margin-x <px>
    #[arg(long, default_value_t = 4)]
    pub label_margin_x: i32,

    // set the label margin-y <px>
    #[arg(long, default_value_t = 2)]
    pub label_margin_y: i32,
}
