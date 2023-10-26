use std::str::FromStr;

use crate::cli::Args;

// stolen from https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
#[derive(Debug, PartialEq)]
struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[0..2], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[2..4], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[4..6], 16)?;

        Ok(RGB { r, g, b })
    }
}

pub fn args_to_css(args: &Args) -> String {
    let window_bg = RGB::from_str(args.window_background_color.as_ref().unwrap())
        .expect("invalid color for window_background_color");

    let label_bg = RGB::from_str(args.label_background_color.as_ref().unwrap())
        .expect("invalid color for label_background_color");
    let label_fg = RGB::from_str(args.label_text_color.as_ref().unwrap())
        .expect("invalid color for label_text_color");

    let focused_bg = RGB::from_str(args.focused_background_color.as_ref().unwrap())
        .expect("invalid color for focused_background_color");
    let focused_fg = RGB::from_str(args.focused_text_color.as_ref().unwrap())
        .expect("invalid color for focused_text_color");

    format!(
        r#"
        window {{
            background: rgba({}, {}, {}, {});
        }}

        window label {{
            background: rgba({}, {}, {}, {});
            color: rgb({}, {}, {});
            font-family: {};
            font-weight: {};
            font-size: {};
            padding: {}px {}px;
        }}

        .focused {{
            background: rgba({}, {}, {}, {});
            color: rgb({}, {}, {})
        }}
        "#,
        window_bg.r,
        window_bg.g,
        window_bg.b,
        args.window_background_opacity.unwrap(),
        label_bg.r,
        label_bg.g,
        label_bg.b,
        args.label_background_opacity.unwrap(),
        label_fg.r,
        label_fg.g,
        label_fg.b,
        args.font_family.as_ref().unwrap(),
        args.font_weight.as_ref().unwrap(),
        args.font_size.as_ref().unwrap(),
        args.label_padding_y.unwrap(),
        args.label_padding_x.unwrap(),
        focused_bg.r,
        focused_bg.g,
        focused_bg.b,
        args.focused_background_opacity.unwrap(),
        focused_fg.r,
        focused_fg.g,
        focused_fg.b,
    )
}
