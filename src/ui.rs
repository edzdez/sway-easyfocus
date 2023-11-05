use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use gtk::{prelude::*, Application, CssProvider, StyleContext};
use swayipc::{Connection, Node};

use crate::{cli::Args, sway, utils};

fn calculate_geometry(window: &Node, output: &Node, args: Arc<Args>) -> (i32, i32) {
    // TODO: this doesn't work properly with stacked windows
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;

    let anchor_x = output.rect.x;
    let anchor_y = output.rect.y;

    let rel_x = rect.x + window_rect.x + deco_rect.x + args.label_margin_x.unwrap();
    let rel_y = rect.y - (deco_rect.height - args.label_margin_y.unwrap());

    (rel_x - anchor_x, rel_y - anchor_y)
}

// TODO: Currently this only closes the windows on a single output.
// It needs to close the windows on all three outputs
fn handle_keypress(key_to_con_id: &HashMap<char, i64>, keyval: &str) {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            let con_id = key_to_con_id[&c];
            sway::focus(con_id);
        }
        std::process::exit(0);
    }
}

fn build_ui(app: &Application, args: Arc<Args>, conn: Arc<Mutex<Connection>>) {

    // get windows from sway
    let output_nodes = sway::get_all_output_nodes(conn.clone());

    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    // Create one "window" for each output each with its own keypress handler.
    for output in output_nodes {
        let workspace = sway::get_focused_workspace(&output);
        let sway_windows = sway::get_window_nodes_for_one_workspace(&workspace);

        // exit if no sway_windows open
        if sway_windows.len() == 0 {
            return;
        }

        // TODO: Every GTK is getting opened on the current output
        // Use the Sway "output" details to decide where to place the window.
        let window = gtk::ApplicationWindow::new(app);

        // before the window is first realized, set it up to be a layer surface
        gtk_layer_shell::init_for_window(&window);
        // display it above normal sway_windows
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

        // receive keyboard events from the compositor
        gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);

        // take up the full screen
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

        let fixed = gtk::Fixed::new();
        // map keys to window Ids
        let mut key_to_con_id = HashMap::new();

        for (_idx, window) in sway_windows.iter().enumerate() {
            let (x, y) = calculate_geometry(window, &output, args.clone());
            let label = gtk::Label::new(Some(""));
            let letter = chars.next().unwrap();
            key_to_con_id.insert(letter, window.id);
            label.set_markup(&format!("{}", letter));
            fixed.put(&label, x, y);

            // Apply a CSS class to the focused window so it can be styled differently
            if window.focused {
                label.style_context().add_class("focused");
            }
        }

        // TODO: close windows on all outputs
        window.connect_key_press_event(move |window, event| {
            let keyval = event
                .keyval()
                .name()
                .expect("the key pressed does not have a name?");
            handle_keypress(&key_to_con_id, &keyval);
            window.close();
            Inhibit(false)
        });

        window.add(&fixed);
        window.show_all();
    }
}

fn load_css(args: Arc<Args>) {
    let provider = CssProvider::new();
    provider
        .load_from_data(utils::args_to_css(&args).as_bytes())
        .expect("failed to load css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        // we can unwrap because there should be a default screen
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(conn: Arc<Mutex<Connection>>, args: Arc<Args>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    let args_clone = args.clone();
    app.connect_startup(move |_| load_css(args_clone.clone()));
    app.connect_activate(move |app| {
        build_ui(app, args.clone(), conn.clone());
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
