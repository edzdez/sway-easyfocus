use std::sync::{Arc, Mutex};

use gtk::{prelude::*, Application, CssProvider, StyleContext};
use swayipc::{Connection, Node};

use crate::sway;

// TODO: make this not hardcoded (clap?)
const PADDING_X: i32 = 4;
const PADDING_Y: i32 = 2;

fn calculate_geometry(window: &Node) -> (i32, i32) {
    // TODO: this doesn't work properly with stacked windows
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;
    dbg!(&window);

    let x = rect.x + window_rect.x + deco_rect.x + PADDING_X;
    let y = rect.y - (deco_rect.height - PADDING_Y);

    (x, y)
}

fn handle_keypress(conn: Arc<Mutex<Connection>>, windows: &[Node], keyval: &str) {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            // this is kinda hacky
            let c_index = c as usize - 'a' as usize;
            if c_index < windows.len() {
                sway::focus(conn, windows, c_index);
            }
        }
    }
}

fn build_ui(app: &Application, conn: Arc<Mutex<Connection>>) {
    // get windows from sway
    let workspace = sway::get_focused_workspace(conn.clone());
    let windows = sway::get_all_windows(&workspace);

    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);
    // display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // receive keyboard events from the compositor
    gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);

    let windows_clone = windows.clone();
    window.connect_key_press_event(move |window, event| {
        let keyval = event
            .keyval()
            .name()
            .expect("the key pressed does not have a name?");
        handle_keypress(conn.clone(), &windows_clone, &keyval);
        window.close();
        Inhibit(false)
    });

    // take up the full screen
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

    let fixed = gtk::Fixed::new();

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
        .expect("failed to load css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        // we can unwrap because there should be a default screen
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(conn: Arc<Mutex<Connection>>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| {
        build_ui(app, conn.clone());
    });

    app.run();
}
