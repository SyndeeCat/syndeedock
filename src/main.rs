use tokio;

use gtk::{prelude::*, Application, ApplicationWindow, Box, Button, Orientation};

fn draw_dock(application: &Application) {
    let window = ApplicationWindow::new(application);

    gtk_layer_shell::init_for_window(&window);

    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, 850);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, 900);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 10);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, 10);

    let anchors = [
        (gtk_layer_shell::Edge::Left, true),
        (gtk_layer_shell::Edge::Right, true),
        (gtk_layer_shell::Edge::Top, false),
        (gtk_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    let hbox = Box::new(Orientation::Horizontal, 5);

    let menu_button = Button::builder().label("MENU").build();
    menu_button.connect_clicked(|_| {
        eprintln!("Menu opened!");
    });

    let test_button = Button::builder().label("TEST").build();
    test_button.connect_clicked(|_| {
        eprintln!("Just test!");
    });

    hbox.add(&menu_button);
    hbox.add(&test_button);

    window.add(&hbox);
    window.set_border_width(8);
    window.show_all()
}

#[tokio::main]
async fn main() {
    let application = gtk::Application::builder()
        .application_id("org.syndee.SyndeeDock")
        .build();

    application.connect_activate(|app| {
        draw_dock(app);
    });

    application.run();
}
