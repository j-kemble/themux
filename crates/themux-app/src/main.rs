// themux-app: GTK4 application binary.
//
// The graphical frontend that ties together:
// - Ghostty GTK terminal widget (GPU-accelerated rendering)
// - WebKitGTK browser widget (in-app browser)
// - Sidebar with vertical workspace tabs
// - Split pane layout via GtkPaned
// - Socket server (runs in background)
// - Notification integration

mod app;
mod ui;

use gtk4::prelude::*;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "themux=info".into()),
        )
        .init();

    tracing::info!("themux GUI starting v{}", env!("CARGO_PKG_VERSION"));

    let application = gtk4::Application::new(
        Some("app.themux.Themux"),
        gtk4::gio::ApplicationFlags::default(),
    );

    application.connect_activate(|app| {
        if let Err(e) = app::build_ui(app) {
            tracing::error!("Failed to build UI: {}", e);
        }
    });

    application.run();
}
