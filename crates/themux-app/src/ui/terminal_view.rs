// Ghostty GTK terminal view integration.

use gtk4::prelude::*;
use gtk4::Box as GtkBox;

use crate::ui::terminal::TerminalWidget;

pub fn new() -> GtkBox {
    let container = GtkBox::new(gtk4::Orientation::Vertical, 0);
    container.set_vexpand(true);
    container.set_hexpand(true);

    let widget = match TerminalWidget::new(80, 24, 8, 16) {
        Ok(t) => t,
        Err(e) => {
            let label = gtk4::Label::new(Some(&format!("Failed to initialize terminal: {}", e)));
            label.add_css_class("terminal-error");
            label.set_vexpand(true);
            label.set_hexpand(true);
            container.append(&label);
            return container;
        }
    };

    container.append(widget.widget());
    container
}