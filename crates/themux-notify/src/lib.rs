// themux-notify: Desktop notification integration.
//
// Two notification paths:
// 1. OSC terminal sequences (OSC 9/99/777) parsed from terminal output
// 2. Socket API (notification.create) from agent hooks
//
// Uses notify-rust for cross-platform desktop notifications (libnotify on Linux).

pub mod notification;
pub mod osc;

use notify_rust::Notification as DesktopNotification;

/// Send a desktop notification.
pub fn send_desktop_notification(
    title: &str,
    subtitle: Option<&str>,
    body: Option<&str>,
    sound: Option<&str>,
) -> Result<(), notify_rust::error::Error> {
    let mut notif = DesktopNotification::new();
    notif.summary(title);

    if let Some(sub) = subtitle {
        notif.appname(sub);
    }

    let combined_body = match (subtitle, body) {
        (Some(s), Some(b)) => format!("{}\n{}", s, b),
        (Some(s), None) => s.to_string(),
        (None, Some(b)) => b.to_string(),
        (None, None) => String::new(),
    };

    if !combined_body.is_empty() {
        notif.body(&combined_body);
    }

    // Set sound hint
    if let Some(sound_name) = sound {
        notif.hint(notify_rust::Hint::SoundName(sound_name.to_string()));
    }

    notif.show()?;
    Ok(())
}
