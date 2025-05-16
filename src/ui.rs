use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gtk::{prelude::*, Application, CssProvider, StyleContext, glib};
use swayipc::{Connection, Node, NodeLayout};

use crate::{cli::Args, cli::Command, sway, utils};

fn calculate_geometry(window: &Node, output: &Node, args: Arc<Args>) -> (i32, i32) {
    // dbg!(&window);
    let rect = window.rect;
    let window_rect = window.window_rect;
    let deco_rect = window.deco_rect;

    let anchor_x = output.rect.x;
    let anchor_y = output.rect.y;

    let rel_x = rect.x + window_rect.x + deco_rect.x + args.label_margin_x.unwrap();
    let rel_y = rect.y - (deco_rect.height - args.label_margin_y.unwrap())
        + if window.layout == NodeLayout::Stacked {
            deco_rect.y
        } else {
            0
        };

    (rel_x - anchor_x, rel_y - anchor_y)
}

fn handle_keypress(
    conn: Arc<Mutex<Connection>>,
    key_to_con_id: &HashMap<char, i64>,
    keyval: &str,
    command: &Command,
) -> bool {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            if let Some(con_id) = key_to_con_id.get(&c) {
                match &command {
                    Command::Focus => {
                        sway::focus(conn, *con_id);
                        return true;
                    }
                    Command::Swap { focus } => {
                        sway::swap(conn.clone(), *con_id);

                        if *focus {
                            sway::focus(conn, *con_id);
                        }
                        return true;
                    }
                    Command::Print => {
                        println!("{}", con_id);
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn build_ui(app: &Application, args: Arc<Args>, conn: Arc<Mutex<Connection>>) {
    // get windows from sway
    let output = sway::get_focused_output(conn.clone());
    let workspace = sway::get_focused_workspace(&output);
    let windows = sway::get_all_windows(&workspace);

    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    // exit if no windows open
    if windows.is_empty() {
        return;
    }

    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);
    // display it above normal windows
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

    for (_idx, window) in windows.iter().enumerate() {
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

    let key_to_con_id_clone = key_to_con_id.clone();
    let args_clone = args.clone();
    
    window.connect_key_press_event(move |window, event| {
        let keyval = event
            .keyval()
            .name()
            .expect("the key pressed does not have a name?");
            
        let window_focused = handle_keypress(
            conn.clone(),
            &key_to_con_id_clone,
            &keyval,
            &args_clone.command.unwrap_or(Command::Focus),
        );
        
        if window_focused {
            // Keep window open for 500ms to show visual confirmation
            let window_clone = window.clone();
            glib::timeout_add_local(Duration::from_millis(500), move || {
                window_clone.close();
                glib::Continue(false)
            });

            // Find the window label that was selected and highlight it
            if let Some(fixed) = window.child().and_then(|c| c.downcast::<gtk::Fixed>().ok()) {
                let c = keyval.chars().next().unwrap();
                
                for child in fixed.children() {
                    // Remove any existing styling
                    child.style_context().remove_class("focused");
                    
                    // This assumes labels were added in the same order as windows
                    if let Some(label) = child.downcast_ref::<gtk::Label>() {
                        let label_text = label.text();
                        let label_char = label_text.chars().next().unwrap();
                        
                        if label_char == c {
                            // Apply special highlight class to the selected window
                            child.style_context().add_class("selected");
                        }
                    }
                }
            }
            
            Inhibit(true) // Prevent default handler from closing window
        } else {
            window.close();
            Inhibit(false)
        }
    });

    window.add(&fixed);
    window.show_all();
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