use anyhow::Result;
use gtk::{prelude::*, Application, CssProvider, StyleContext};
use swayipc::{Connection, Node};

use crate::sway;

const PADDING_X: i32 = 4;
const PADDING_Y: i32 = 2;

fn calculate_geometry(window: &Node) -> (i32, i32) {
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;

    let x = rect.x + window_rect.x + deco_rect.x + PADDING_X;
    let y = rect.y - (deco_rect.height - PADDING_Y);

    (x, y)
}

fn build_ui(app: &Application, workspace: &Node) {
    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);
    // display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // receive keyboard events from the compositor
    gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);
    window.connect_key_press_event(|window, event| {
        let keyval = event.keyval().name().unwrap();
        if keyval == "Escape" {
            window.close();
        } else if keyval.len() == 1 && keyval.chars().next().unwrap().is_alphabetic() {
            dbg!(&keyval);
        }

        Inhibit(false)
    });

    // take up the full screen
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

    let fixed = gtk::Fixed::new();

    let windows = sway::get_all_windows(workspace);
    for (idx, window) in windows.iter().enumerate() {
        let (x, y) = calculate_geometry(window);
        let label = gtk::Label::new(Some(""));
        // TODO: make this work for workspaces with more than 26 windows
        label.set_markup(&format!("{}", ('a' as usize + idx % 26) as u8 as char));
        fixed.put(&label, x, y);
    }

    window.add(&fixed);

    window.show_all();
}

fn load_css() {
    // TODO: make this customizable (perhaps clap?)
    let provider = CssProvider::new();
    provider
        .load_from_data(include_bytes!("style.css"))
        .unwrap();

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(mut conn: Connection) -> Result<()> {
    let focused_workspace = sway::get_focused_workspace(&mut conn)?;

    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        build_ui(app, &focused_workspace);
    });

    app.run();

    Ok(())
}
