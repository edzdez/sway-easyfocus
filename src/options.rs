use std::str::FromStr;

use crate::{
    cli::{Args, Command},
    util::Rgb,
};

#[derive(Debug)]
pub struct Options {
    pub chars: String,
    pub window_background_color: Rgb,
    pub window_background_opacity: f64,
    pub label_background_color: Rgb,
    pub label_background_opacity: f64,
    pub label_text_color: Rgb,
    pub focused_background_color: Rgb,
    pub focused_background_opacity: f64,
    pub focused_text_color: Rgb,
    pub font_family: String,
    pub font_weight: String,
    pub font_size: String,
    pub label_padding_x: i32,
    pub label_padding_y: i32,
    pub label_margin_x: i32,
    pub label_margin_y: i32,
    pub show_confirmation: bool,
    pub command: Command,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            chars: "fjghdkslaemuvitywoqpcbnxz".to_string(),
            window_background_color: Rgb::from_str("1d1f21").unwrap(),
            window_background_opacity: 0.2,
            label_background_color: Rgb::from_str("1d1f21").unwrap(),
            label_background_opacity: 1.0,
            label_text_color: Rgb::from_str("c5c8c6").unwrap(),
            focused_background_color: Rgb::from_str("285577").unwrap(),
            focused_background_opacity: 1.0,
            focused_text_color: Rgb::from_str("ffffff").unwrap(),
            font_family: "monospace".to_string(),
            font_weight: "bold".to_string(),
            font_size: "medium".to_string(),
            label_padding_x: 4,
            label_padding_y: 0,
            label_margin_x: 4,
            label_margin_y: 2,
            show_confirmation: true,
            command: Command::Focus,
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use figment::{
        providers::{Format, Yaml},
        Figment,
    };

    use super::*;

    #[test]
    fn config_without_show_confirmation_keeps_default_true() {
        let args = Figment::new()
            .merge(Yaml::string("chars: abc\n"))
            .extract::<Args>()
            .unwrap();
        let mut opts = Options::default();

        opts.merge(&args);

        assert_eq!(opts.chars, "abc");
        assert!(opts.show_confirmation);
    }

    #[test]
    fn config_can_disable_show_confirmation() {
        let args = Figment::new()
            .merge(Yaml::string("show_confirmation: false\n"))
            .extract::<Args>()
            .unwrap();
        let mut opts = Options::default();

        opts.merge(&args);

        assert!(!opts.show_confirmation);
    }

    #[test]
    fn absent_cli_flag_keeps_default_true() {
        let args = Args::try_parse_from(["sway-easyfocus"]).unwrap();
        let mut opts = Options::default();

        opts.merge(&args);

        assert!(opts.show_confirmation);
    }

    #[test]
    fn cli_flag_sets_show_confirmation_true() {
        let args = Args::try_parse_from(["sway-easyfocus", "--show-confirmation"]).unwrap();

        assert_eq!(args.show_confirmation, Some(true));
    }
}

impl Options {
    pub fn merge(&mut self, args: &Args) {
        macro_rules! update {
            ($field:ident) => {
                if let Some(ref v) = args.$field {
                    self.$field = v.clone();
                }
            };
        }

        update!(chars);
        update!(window_background_color);
        update!(window_background_opacity);
        update!(label_background_color);
        update!(label_background_opacity);
        update!(label_text_color);
        update!(focused_background_color);
        update!(focused_background_opacity);
        update!(focused_text_color);
        update!(font_family);
        update!(font_weight);
        update!(font_size);
        update!(label_padding_x);
        update!(label_padding_y);
        update!(label_margin_x);
        update!(label_margin_y);
        update!(show_confirmation);
        update!(command);
    }

    pub fn to_css(&self) -> String {
        format!(
            r#"
            window {{
                background: rgba({},{});
            }}

            window label {{
                background: rgba({},{});
                color: rgb({});
                font-family: {};
                font-weight: {};
                font-size: {};
                padding: {}px {}px;
            }}

            .focused {{
                background: rgba({},{});
                color: rgb({});
            }}
            "#,
            self.window_background_color,
            self.window_background_opacity,
            self.label_background_color,
            self.label_background_opacity,
            self.label_text_color,
            self.font_family,
            self.font_weight,
            self.font_size,
            self.label_padding_y,
            self.label_padding_x,
            self.focused_background_color,
            self.focused_background_opacity,
            self.focused_text_color,
        )
    }
}
