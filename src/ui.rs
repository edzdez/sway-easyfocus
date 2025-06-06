use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

use gtk4::{prelude::*, Application, CssProvider, glib};
use gtk4::glib::ControlFlow;
use gtk4_layer_shell as gtk_layer_shell;
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
    let output_nodes = sway::get_all_output_nodes(conn.clone());

    // Shared state for all monitors
    let all_key_to_con_id: Rc<RefCell<HashMap<char, i64>>> = Rc::new(RefCell::new(HashMap::new()));
    let all_windows: Rc<RefCell<Vec<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(Vec::new()));
    let mut all_windows_map: HashMap<i64, (Node, Node, char)> = HashMap::new();

    // Get global character sequence
    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    // Process each output
    for output in output_nodes {
        let workspace = sway::get_focused_workspace(&output);
        let windows = sway::get_all_windows(&workspace);

        // Skip empty workspaces
        if windows.is_empty() {
            continue;
        }

        // Create GTK window for this output
        let window = gtk4::ApplicationWindow::new(app);

        // Configure layer shell
        // Setting a namespace allows WM rules to target these windows.
        gtk_layer_shell::LayerShell::init_layer_shell(&window);
        gtk_layer_shell::LayerShell::set_namespace(&window, Some("sway-easyfocus"));
        gtk_layer_shell::LayerShell::set_layer(&window, gtk_layer_shell::Layer::Overlay);
        // Not sure why you'd want to use the Top layer instead, but here's the syntax.
        // gtk_layer_shell::LayerShell::set_layer(&window, gtk_layer_shell::Layer::Top);
        gtk_layer_shell::LayerShell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);
        gtk_layer_shell::LayerShell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
        gtk_layer_shell::LayerShell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
        gtk_layer_shell::LayerShell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::LayerShell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

        // Set monitor for this output
        let display = gtk4::gdk::Display::default().unwrap();
        let monitors = display.monitors();
        for i in 0..monitors.n_items() {
            if let Some(monitor) = monitors.item(i).and_then(|obj| obj.downcast::<gtk4::gdk::Monitor>().ok()) {
                let geometry = monitor.geometry();
                if geometry.x() <= output.rect.x && 
                   output.rect.x < geometry.x() + geometry.width() &&
                   geometry.y() <= output.rect.y &&
                   output.rect.y < geometry.y() + geometry.height() {
                    gtk_layer_shell::LayerShell::set_monitor(&window, Some(&monitor));
                    break;
                }
            }
        }

        let fixed = gtk4::Fixed::new();
        let mut local_key_to_con_id = HashMap::new();

        // Create labels for windows
        for (_idx, window_node) in windows.iter().enumerate() {
            let (x, y) = calculate_geometry(window_node, &output, args.clone());
            let label = gtk4::Label::new(Some(""));

            let letter = match chars.next() {
                Some(c) => c,
                None => 'a', // Fallback if we run out of characters
            };

            // Store mappings
            local_key_to_con_id.insert(letter, window_node.id);
            all_key_to_con_id.borrow_mut().insert(letter, window_node.id);
            all_windows_map.insert(window_node.id, (window_node.clone(), output.clone(), letter));

            label.set_markup(&format!("{}", letter));

            // Ensure labels are visible and properly sized on the overlay
            label.set_halign(gtk4::Align::Center);
            label.set_valign(gtk4::Align::Center);

            fixed.put(&label, x as f64, y as f64);

            if window_node.focused {
                label.add_css_class("focused");
            }
        }

        // Set up key handler - use global key map for both single and multi-monitor
        let key_map = all_key_to_con_id.clone();

        let all_windows_clone = all_windows.clone();
        let args_clone = args.clone();
        let conn_clone = conn.clone();
        let all_windows_map_clone = all_windows_map.clone();

        // GTK 4 uses EventControllerKey for keyboard input
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            let keyval_name = keyval.name();
            if let Some(keyval_str) = keyval_name {
                let keyval_str = keyval_str.as_str();

                let window_focused = handle_keypress(
                    conn_clone.clone(),
                    &key_map.borrow(),
                    keyval_str,
                    &args_clone.command.unwrap_or(Command::Focus),
                );

                if window_focused {
                    let c = keyval_str.chars().next().unwrap();
                    let show_confirmation = args_clone.show_confirmation.unwrap_or(true);

                    // Find and update the selected label, hide all other labels
                    if show_confirmation {
                        if let Some(con_id) = key_map.borrow().get(&c) {
                            if let Some((_, _, _)) = all_windows_map_clone.get(con_id) {
                                // Find the label for this character across all windows
                                for window in all_windows_clone.borrow().iter() {
                                    let mut found_selected_label = false;
                                    if let Some(fixed) = window.child().and_then(|c| c.downcast::<gtk4::Fixed>().ok()) {
                                        let mut child = fixed.first_child();
                                        while let Some(widget) = child {
                                            child = widget.next_sibling(); // Get next sibling before moving widget
                                            if let Ok(label) = widget.downcast::<gtk4::Label>() {
                                                if label.text() == c.to_string() {
                                                    // Update the CSS class to reflect the focus has changed.
                                                    label.add_css_class("focused");
                                                    found_selected_label = true;
                                                } else {
                                                    // Hide all other labels
                                                    label.set_visible(false);
                                                }
                                            }
                                        }
                                    }
                                    // Hide windows that don't contain the selected label
                                    if !found_selected_label {
                                        window.set_visible(false);
                                    }
                                }
                            }
                        }
                    } else {
                        // If no confirmation, hide all windows immediately
                        for w in all_windows_clone.borrow().iter() {
                            w.set_visible(false);
                        }
                    }

                    // Close all windows after delay (or immediately if no confirmation)
                    let windows_to_close = all_windows_clone.borrow().clone();
                    let delay = if show_confirmation { 500 } else { 0 };
                    glib::timeout_add_local(Duration::from_millis(delay), move || {
                        for w in windows_to_close.iter() {
                            w.close();
                        }
                        ControlFlow::Break
                    });

                    glib::Propagation::Stop
                } else {
                    // Close windows on escape or invalid key
                    for w in all_windows_clone.borrow().iter() {
                        w.close();
                    }
                    glib::Propagation::Stop
                }
            } else {
                glib::Propagation::Proceed
            }
        });

        window.add_controller(key_controller);
        window.set_child(Some(&fixed));
        all_windows.borrow_mut().push(window);
    }

    // Show all windows
    for window in all_windows.borrow().iter() {
        window.present();
    }
}

fn load_css(args: Arc<Args>) {
    let provider = CssProvider::new();
    provider.load_from_data(&utils::args_to_css(&args));

    // Add the provider to the default display
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
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
