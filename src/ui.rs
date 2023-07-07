use gtk::{prelude::*, Application};

fn build_ui(app: &Application) {
    // create a normal gtk window
    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);

    // display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // Set up a widget
    let label = gtk::Label::new(Some(""));
    label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    window.add(&label);
    window.set_border_width(12);
    window.show_all()
}

pub fn run_ui() {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
