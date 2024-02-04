use std::sync::{Arc, Mutex};

use clap::Parser;

use figment::{
    providers::{Format, Yaml},
    Figment,
};

use cli::Args;

mod cli;
mod sway;
mod ui;
mod utils;

fn parse_config() -> Arc<Args> {
    // there is probably a way better way to do this...
    let mut args = Args::default();

    let base_dirs = xdg::BaseDirectories::with_prefix("sway-easyfocus").unwrap();
    let config_path = base_dirs
        .place_config_file("config.yaml")
        .expect("failed to create config directory");

    if let Ok(config_args) = Figment::new()
        .merge(Yaml::file(&config_path))
        .extract::<Args>()
    {
        args.merge(&config_args);
        dbg!(&config_args);
    }

    let cli_args = Args::parse();
    args.merge(&cli_args);

    Arc::new(args)
}

fn main() {
    let args = parse_config();

    let conn = Arc::new(Mutex::new(sway::acquire_connection()));
    ui::run_ui(conn, args);
}
