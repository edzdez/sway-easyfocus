use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, Application, CssProvider, StyleContext, glib};
use gtk_layer_shell;
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

fn create_confirmation_window(
    app: &Application,
    args: Arc<Args>,
    window_node: &Node,
    output: &Node,
    selected_char: char,
) {
    let confirm_window = gtk::ApplicationWindow::new(app);

    // Configure as layer shell
    gtk_layer_shell::init_for_window(&confirm_window);
    gtk_layer_shell::set_layer(&confirm_window, gtk_layer_shell::Layer::Overlay);
    gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Right, true);

    // Set on correct monitor
    let display = gtk::gdk::Display::default().unwrap();
    let center_x = output.rect.x + (output.rect.width / 2);
    let center_y = output.rect.y + (output.rect.height / 2);
    if let Some(monitor) = display.monitor_at_point(center_x, center_y) {
        gtk_layer_shell::set_monitor(&confirm_window, &monitor);
    }

    // Add label at correct position
    let (x, y) = calculate_geometry(window_node, output, args);
    let fixed = gtk::Fixed::new();
    let label = gtk::Label::new(Some(&selected_char.to_string()));
    label.style_context().add_class("selected");
    fixed.put(&label, x, y);

    confirm_window.add(&fixed);
    confirm_window.show_all();

    // Set timeout to close
    glib::timeout_add_local(Duration::from_millis(500), move || {
        confirm_window.close();
        glib::Continue(false)
    });
}

fn build_ui(app: &Application, args: Arc<Args>, conn: Arc<Mutex<Connection>>) {
    // Determine if we're in single or multi-monitor mode
    let output_nodes = sway::get_all_output_nodes(conn.clone());
    let is_multi_monitor = output_nodes.len() > 1;

    // Get the outputs to process
    let outputs_to_process = if is_multi_monitor {
        output_nodes
    } else {
        vec![sway::get_focused_output(conn.clone())]
    };

    // Shared state for multi-monitor mode
    let all_key_to_con_id: Rc<RefCell<HashMap<char, i64>>> = Rc::new(RefCell::new(HashMap::new()));
    let all_windows: Rc<RefCell<Vec<gtk::ApplicationWindow>>> = Rc::new(RefCell::new(Vec::new()));
    let mut all_windows_map: HashMap<i64, (Node, Node, char)> = HashMap::new();

    // Get global character sequence
    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    // Process each output
    for output in outputs_to_process {
        let workspace = sway::get_focused_workspace(&output);
        let windows = sway::get_all_windows(&workspace);

        // Skip empty workspaces
        if windows.is_empty() {
            continue;
        }

        // Create GTK window for this output
        let window = gtk::ApplicationWindow::new(app);

        // Configure layer shell
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);
        gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

        // Set monitor for multi-monitor mode
        if is_multi_monitor {
            let display = gtk::gdk::Display::default().unwrap();
            let center_x = output.rect.x + (output.rect.width / 2);
            let center_y = output.rect.y + (output.rect.height / 2);
            if let Some(monitor) = display.monitor_at_point(center_x, center_y) {
                gtk_layer_shell::set_monitor(&window, &monitor);
            }
        }

        let fixed = gtk::Fixed::new();
        let mut local_key_to_con_id = HashMap::new();

        // Create labels for windows
        for (_idx, window_node) in windows.iter().enumerate() {
            let (x, y) = calculate_geometry(window_node, &output, args.clone());
            let label = gtk::Label::new(Some(""));

            let letter = match chars.next() {
                Some(c) => c,
                None => 'a', // Fallback if we run out of characters
            };

            // Store mappings
            local_key_to_con_id.insert(letter, window_node.id);
            all_key_to_con_id.borrow_mut().insert(letter, window_node.id);
            all_windows_map.insert(window_node.id, (window_node.clone(), output.clone(), letter));

            label.set_markup(&format!("{}", letter));
            fixed.put(&label, x, y);

            if window_node.focused {
                label.style_context().add_class("focused");
            }
        }

        // Set up key handler
        let key_map = if is_multi_monitor {
            all_key_to_con_id.clone()
        } else {
            Rc::new(RefCell::new(local_key_to_con_id))
        };

        let all_windows_clone = all_windows.clone();
        let args_clone = args.clone();
        let conn_clone = conn.clone();
        let app_ref = app.clone();
        let all_windows_map_clone = all_windows_map.clone();

        window.connect_key_press_event(move |current_window, event| {
            let keyval = event
                .keyval()
                .name()
                .expect("the key pressed does not have a name?");

            let window_focused = handle_keypress(
                conn_clone.clone(),
                &key_map.borrow(),
                &keyval,
                &args_clone.command.unwrap_or(Command::Focus),
            );

            if window_focused {
                let c = keyval.chars().next().unwrap();
                let show_confirmation = args_clone.show_confirmation.unwrap_or(true);

                if is_multi_monitor {
                    // Hide all windows immediately
                    for w in all_windows_clone.borrow().iter() {
                        w.hide();
                    }

                    // Create confirmation window if enabled
                    if show_confirmation {
                        if let Some(con_id) = key_map.borrow().get(&c) {
                            if let Some((node, output, _)) = all_windows_map_clone.get(con_id) {
                                create_confirmation_window(&app_ref, args_clone.clone(), node, output, c);
                            }
                        }
                    }

                    // Close all windows after delay (or immediately if no confirmation)
                    let windows_to_close = all_windows_clone.borrow().clone();
                    let delay = if show_confirmation { 500 } else { 0 };
                    glib::timeout_add_local(Duration::from_millis(delay), move || {
                        for w in windows_to_close.iter() {
                            w.close();
                        }
                        glib::Continue(false)
                    });
                } else {
                    // Single monitor mode: create confirmation and close current window
                    if show_confirmation {
                        if let Some(con_id) = key_map.borrow().get(&c) {
                            if let Some((node, output, _)) = all_windows_map_clone.get(con_id) {
                                create_confirmation_window(&app_ref, args_clone.clone(), node, output, c);
                            }
                        }
                    }

                    let delay = if show_confirmation { 500 } else { 0 };
                    glib::timeout_add_local(Duration::from_millis(delay), {
                        let window = current_window.clone();
                        move || {
                            window.close();
                            glib::Continue(false)
                        }
                    });
                }

                Inhibit(true)
            } else {
                // Close windows on escape or invalid key
                if is_multi_monitor {
                    for w in all_windows_clone.borrow().iter() {
                        w.close();
                    }
                } else {
                    current_window.close();
                }
                Inhibit(false)
            }
        });

        window.add(&fixed);
        all_windows.borrow_mut().push(window);
    }

    // Show all windows
    for window in all_windows.borrow().iter() {
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
