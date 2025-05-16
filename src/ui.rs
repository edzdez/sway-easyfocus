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
    
    let app = app.clone();
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
            let c = keyval.chars().next().unwrap();
            
            // Find the window label that was selected
            if let Some(_fixed) = window.child().and_then(|c| c.downcast::<gtk::Fixed>().ok()) {
                // Create a new confirmation window
                let confirm_window = gtk::ApplicationWindow::new(&app);
                gtk_layer_shell::init_for_window(&confirm_window);
                gtk_layer_shell::set_layer(&confirm_window, gtk_layer_shell::Layer::Overlay);
                gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Top, true);
                gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Bottom, true);
                gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Left, true);
                gtk_layer_shell::set_anchor(&confirm_window, gtk_layer_shell::Edge::Right, true);

                // Explicitly set the monitor for the confirmation window
                let display = gtk::gdk::Display::default().unwrap();
                let center_x = output.rect.x + (output.rect.width / 2);
                let center_y = output.rect.y + (output.rect.height / 2);
                if let Some(monitor) = display.monitor_at_point(center_x, center_y) {
                    gtk_layer_shell::set_monitor(&confirm_window, &monitor);
                }
                
                // Set up a fixed container for the confirmation label
                let confirm_fixed = gtk::Fixed::new();
                
                // Find the position of the selected label
                let mut x_pos = 0;
                let mut y_pos = 0;
                
                for (idx, win) in windows.iter().enumerate() {
                    let letter = ('a' as u8 + idx as u8 % 26) as char;
                    if letter == c {
                        // Found the matching window
                        let (x, y) = calculate_geometry(win, &output, args.clone());
                        x_pos = x;
                        y_pos = y;
                        break;
                    }
                }
                
                // Create the confirmation label
                let confirm_label = gtk::Label::new(Some(&c.to_string()));
                confirm_label.style_context().add_class("selected");
                confirm_fixed.put(&confirm_label, x_pos, y_pos);
                
                // Add to window and show
                confirm_window.add(&confirm_fixed);
                confirm_window.show_all();
                
                // Set up timer to close the confirmation window
                glib::timeout_add_local(Duration::from_millis(500), move || {
                    confirm_window.close();
                    glib::Continue(false)
                });
            }
            
            // Delay closing the main window to allow the confirmation window to appear
            glib::timeout_add_local(Duration::from_millis(500), {
                let window = window.clone();
                move || {
                    window.close();
                    glib::Continue(false)
                }
            });
            Inhibit(true)
        } else {
            window.close();
            Inhibit(false)
        }
    });

    window.add(&fixed);
    window.show_all();
}

fn build_ui_multi_monitor(app: &Application, args: Arc<Args>, conn: Arc<Mutex<Connection>>, output_nodes: Vec<Node>) {
    // A shared map to store all container IDs from all monitors
    let all_monitors_key_to_con_id: Rc<RefCell<HashMap<char, i64>>> = Rc::new(RefCell::new(HashMap::new()));
    
    // A shared reference for all GTK windows
    let all_windows: Rc<RefCell<Vec<gtk::ApplicationWindow>>> = Rc::new(RefCell::new(Vec::new()));
    
    // Get a global sequence of characters for labels
    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();
    
    // Build a map of all windows across all monitors
    let mut all_windows_map: HashMap<i64, (Node, Node, char)> = HashMap::new();
    
    // Create a window for each output
    for output in output_nodes {
        let workspace = sway::get_focused_workspace(&output);
        let windows = sway::get_all_windows(&workspace);
        
        // Skip empty workspaces
        if windows.is_empty() {
            continue;
        }
        
        // Create a GTK window for this monitor
        let window = gtk::ApplicationWindow::new(app);
        
        // Configure layer shell for this window
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);
        gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);
        
        // Get the monitor geometry for this output
        
        // Set anchors to cover the full monitor
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);
        
        // Set the output to show this window on
        let display = gtk::gdk::Display::default().unwrap();
        let center_x = output.rect.x + (output.rect.width / 2);
        let center_y = output.rect.y + (output.rect.height / 2);
        if let Some(monitor) = display.monitor_at_point(center_x, center_y) {
            gtk_layer_shell::set_monitor(&window, &monitor);
        }
        
        // Create a fixed container for all labels
        let fixed = gtk::Fixed::new();
        
        // Add labels for all windows on this monitor
        for (_idx, window_node) in windows.iter().enumerate() {
            let (x, y) = calculate_geometry(window_node, &output, args.clone());
            let label = gtk::Label::new(Some(""));
            
            // Get the next letter for this window
            let letter = match chars.next() {
                Some(c) => c,
                None => {
                    // If we ran out of letters, start recycling them with a prefix
                    'a' // Placeholder - in a real implementation we would handle this better
                }
            };
            
            // Store the container ID in the global map
            all_monitors_key_to_con_id.borrow_mut().insert(letter, window_node.id);
            
            // Also store in our auxiliary map
            all_windows_map.insert(window_node.id, (window_node.clone(), output.clone(), letter));
            
            // Set the label text
            label.set_markup(&format!("{}", letter));
            fixed.put(&label, x, y);
            
            // Apply styling
            if window_node.focused {
                label.style_context().add_class("focused");
            }
        }
        
        // Add the fixed container to the window
        window.add(&fixed);
        
        // Store the window in our list
        all_windows.borrow_mut().push(window);
    }
    
    // No windows to show? Exit
    if all_windows.borrow().is_empty() {
        return;
    }
    
    // Set up key handlers for all windows
    let key_to_con_id_clone = all_monitors_key_to_con_id.clone();
    let all_windows_clone = all_windows.clone();
    let args_clone = args.clone();
    let conn_clone = conn.clone();
    let app_ref = app.clone();
    
    // We need to register keypress handlers for all windows
    for window in all_windows.borrow().iter() {
        let key_to_con_id_clone = key_to_con_id_clone.clone();
        let all_windows_clone = all_windows_clone.clone();
        let args_clone = args_clone.clone();
        let conn_clone = conn_clone.clone();
        let all_windows_map_clone = all_windows_map.clone();
        let app_ref = app_ref.clone();
        
        window.connect_key_press_event(move |_window, event| {
            let keyval = event
                .keyval()
                .name()
                .expect("the key pressed does not have a name?");
            
            let window_focused = handle_keypress(
                conn_clone.clone(),
                &key_to_con_id_clone.borrow(),
                &keyval,
                &args_clone.command.unwrap_or(Command::Focus),
            );
            
            if window_focused {
                let c = keyval.chars().next().unwrap();
                
                // Immediately hide all original windows
                for w in all_windows_clone.borrow().iter() {
                    w.hide();
                }

                // Delay closing all windows to allow the confirmation window to appear
                let windows_to_close = all_windows_clone.borrow().clone();
                glib::timeout_add_local(Duration::from_millis(500), move || {
                    for w in windows_to_close.iter() {
                        w.close();
                    }
                    glib::Continue(false)
                });
                
                // Find the window node and output for the selected window
                if let Some(con_id) = key_to_con_id_clone.borrow().get(&c) {
                    if let Some((node, output, _)) = all_windows_map_clone.get(con_id) {
                        // Create a new confirmation window for the selected window
                        let confirm_window = gtk::ApplicationWindow::new(&app_ref);
                        
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
                        let (x, y) = calculate_geometry(node, output, args_clone.clone());
                        let fixed = gtk::Fixed::new();
                        let label = gtk::Label::new(Some(&c.to_string()));
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
                }
                
                Inhibit(true)
            } else {
                // Close all windows if Escape or other non-matching key is pressed
                for w in all_windows_clone.borrow().iter() {
                    w.close();
                }
                Inhibit(false)
            }
        });
        
        // Show the window
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
        // Check if we should use multi-monitor mode
        let output_nodes = sway::get_all_output_nodes(conn.clone());
        
        if output_nodes.len() > 1 {
            // Multi-monitor mode
            build_ui_multi_monitor(app, args.clone(), conn.clone(), output_nodes);
        } else {
            // Single monitor mode (legacy)
            build_ui(app, args.clone(), conn.clone());
        }
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
