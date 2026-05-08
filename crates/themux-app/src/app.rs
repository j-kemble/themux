// Application initialization and window management.

use gtk4::prelude::*;

/// Build the main application window.
pub fn build_ui(app: &gtk4::Application) -> Result<(), Box<dyn std::error::Error>> {
    // Load CSS
    let provider = gtk4::CssProvider::new();
    provider.load_from_resource("/app/themux/Themux/style.css");

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Build main window
    let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("themux"));
    window.set_default_size(1200, 800);

    // Main horizontal layout: sidebar | content
    let main_paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);

    // Sidebar (placeholder)
    let sidebar = build_sidebar();
    main_paned.set_start_child(Some(&sidebar));
    main_paned.set_position(260);

    // Content area (placeholder)
    let content = build_content();
    main_paned.set_end_child(Some(&content));

    window.set_child(Some(&main_paned));
    window.present();

    Ok(())
}

fn build_sidebar() -> gtk4::Box {
    let sidebar = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    sidebar.set_width_request(220);
    sidebar.add_css_class("sidebar");

    // Sidebar header
    let header = gtk4::Label::new(Some("Workspaces"));
    header.add_css_class("sidebar-header");
    sidebar.append(&header);

    // Workspace list (placeholder)
    let list = gtk4::ListBox::new();
    list.add_css_class("workspace-list");

    let demo_label = gtk4::Label::new(Some("  No workspaces yet"));
    demo_label.set_halign(gtk4::Align::Start);
    list.append(&demo_label);

    sidebar.append(&list);

    sidebar
}

fn build_content() -> gtk4::Box {
    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    content.add_css_class("content");

    // Terminal view (Ghostty widget placeholder for now)
    let terminal = crate::ui::terminal_view::new();
    terminal.set_vexpand(true);
    terminal.set_hexpand(true);
    content.append(&terminal);

    content
}
