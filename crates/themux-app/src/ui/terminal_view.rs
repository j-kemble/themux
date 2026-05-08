// Ghostty GTK terminal view integration.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label};

/// Create a terminal view widget that will eventually embed the Ghostty GTK widget.
/// For now, returns a GtkBox with a placeholder label.
pub fn new() -> GtkBox {
    let container = GtkBox::new(gtk4::Orientation::Vertical, 0);
    container.set_vexpand(true);
    container.set_hexpand(true);

    let placeholder = Label::new(Some("Ghostty terminal widget placeholder"));
    placeholder.add_css_class("terminal-placeholder");
    placeholder.set_vexpand(true);
    placeholder.set_hexpand(true);
    container.append(&placeholder);

    container
}
