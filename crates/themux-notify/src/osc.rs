// OSC sequence parser for terminal notifications.
//
// Detects OSC 9, OSC 99, and OSC 777 sequences used by cmux
// for in-terminal notification delivery.

/// Parsed OSC notification content.
#[derive(Debug, Clone)]
pub struct OscNotification {
    pub code: u16,
    pub title: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
}

/// Parse an OSC notification from a terminal output line.
///
/// Format:
///   ESC ] 9 ; <message> ST        — simple notification
///   ESC ] 99 ; <message> ST        — notification with title (cmux extension)
///   ESC ] 777 ; <key>=<value> ST  — structured notification (cmux extension)
pub fn parse_osc(line: &str) -> Option<OscNotification> {
    // Strip the OSC prefix: ESC ]
    let content = line.strip_prefix("\x1b]")?;
    // Strip the string terminator: BEL (\x07) or ST (\x1b\\)
    let content = content
        .strip_suffix('\x07')
        .or_else(|| content.strip_suffix("\x1b\\"))?;

    // Split on first ';' to get OSC code vs message
    let (code_str, message) = content.split_once(';')?;
    let code: u16 = code_str.parse().ok()?;

    match code {
        9 => Some(OscNotification {
            code: 9,
            title: Some("Notification".to_string()),
            body: Some(message.to_string()),
            category: None,
        }),
        99 => {
            // OSC 99: message is the notification body
            Some(OscNotification {
                code: 99,
                title: None,
                body: Some(message.to_string()),
                category: None,
            })
        }
        777 => {
            // OSC 777: key=value pairs
            let mut title = None;
            let mut body = None;
            let mut category = None;

            for pair in message.split(';') {
                if let Some((key, value)) = pair.split_once('=') {
                    match key {
                        "title" => title = Some(value.to_string()),
                        "body" => body = Some(value.to_string()),
                        "category" => category = Some(value.to_string()),
                        _ => {}
                    }
                }
            }

            Some(OscNotification {
                code: 777,
                title,
                body,
                category,
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osc_9_simple() {
        let input = "\x1b]9;Hello world\x07";
        let result = parse_osc(input).unwrap();
        assert_eq!(result.code, 9);
        assert_eq!(result.body.unwrap(), "Hello world");
    }

    #[test]
    fn test_osc_777_structured() {
        let input = "\x1b]777;title=Agent Ready;body=Claude is waiting;category=agent\x07";
        let result = parse_osc(input).unwrap();
        assert_eq!(result.code, 777);
        assert_eq!(result.title.unwrap(), "Agent Ready");
        assert_eq!(result.body.unwrap(), "Claude is waiting");
        assert_eq!(result.category.unwrap(), "agent");
    }
}
