// Tmux compatibility layer: translates tmux commands to themux socket calls.
//
// When agents run `tmux new-window`, `tmux split-window`, `tmux send-keys`,
// etc., this module translates them into V2 JSON-RPC method calls.
//
// Mapped commands:
//   new-window      -> workspace.create
//   split-window    -> surface.split
//   send-keys       -> surface.send_text
//   capture-pane    -> surface.read_text
//   resize-pane     -> surface.resize
//   select-pane     -> pane.focus
//   kill-pane       -> surface.close
//   list-panes      -> pane.list
//   display-message -> (no-op / log)

use std::collections::HashMap;

/// Parse tmux arguments and dispatch to the appropriate themux command.
pub fn handle_tmux_compat(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("usage: themux __tmux-compat <tmux-command> [args...]".into());
    }

    let command = &args[0];
    let rest = &args[1..];

    match command.as_str() {
        "new-window" | "neww" => tmux_new_window(rest),
        "split-window" | "splitw" => tmux_split_window(rest),
        "send-keys" | "send" => tmux_send_keys(rest),
        "capture-pane" | "capturep" => tmux_capture_pane(rest),
        "select-pane" | "selectp" => tmux_select_pane(rest),
        "kill-pane" | "killp" => tmux_kill_pane(rest),
        "list-panes" | "lsp" => tmux_list_panes(rest),
        "resize-pane" | "resizep" => tmux_resize_pane(rest),
        "display-message" | "display" => Ok(String::new()),
        "last-window" | "last" => Ok(r#"{"method":"workspace.select","params":{"index":"last"}}"#.into()),
        "next-window" | "next" => Ok(r#"{"method":"workspace.select","params":{"direction":"next"}}"#.into()),
        "previous-window" | "prev" => Ok(r#"{"method":"workspace.select","params":{"direction":"previous"}}"#.into()),
        _ => Err(format!("unsupported tmux command: {}", command)),
    }
}

fn tmux_new_window(args: &[String]) -> Result<String, String> {
    let name = parse_flag(args, "-n");
    let cwd = parse_flag(args, "-c");

    let mut params = serde_json::json!({});
    if let Some(n) = name {
        params["name"] = serde_json::Value::String(n.to_string());
    }
    if let Some(c) = cwd {
        params["cwd"] = serde_json::Value::String(c.to_string());
    }

    Ok(serde_json::json!({
        "method": "workspace.create",
        "params": params
    })
    .to_string())
}

fn tmux_split_window(args: &[String]) -> Result<String, String> {
    let direction = if args.iter().any(|a| a == "-h") {
        "horizontal"
    } else {
        "vertical"
    };

    Ok(serde_json::json!({
        "method": "surface.split",
        "params": {
            "direction": direction,
        }
    })
    .to_string())
}

fn tmux_send_keys(args: &[String]) -> Result<String, String> {
    // Collect text after flags
    let text = args
        .iter()
        .filter(|a| !a.starts_with('-'))
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");

    Ok(serde_json::json!({
        "method": "surface.send_text",
        "params": {
            "text": text,
        }
    })
    .to_string())
}

fn tmux_capture_pane(_args: &[String]) -> Result<String, String> {
    Ok(serde_json::json!({
        "method": "surface.read_text",
        "params": {
            "scrollback": true,
        }
    })
    .to_string())
}

fn tmux_select_pane(args: &[String]) -> Result<String, String> {
    let direction = if args.iter().any(|a| a == "-L") {
        "left"
    } else if args.iter().any(|a| a == "-R") {
        "right"
    } else if args.iter().any(|a| a == "-U") {
        "up"
    } else if args.iter().any(|a| a == "-D") {
        "down"
    } else {
        return Err("select-pane requires -L/-R/-U/-D".into());
    };

    Ok(serde_json::json!({
        "method": "pane.focus",
        "params": {
            "direction": direction,
        }
    })
    .to_string())
}

fn tmux_kill_pane(_args: &[String]) -> Result<String, String> {
    Ok(serde_json::json!({
        "method": "surface.close",
        "params": {}
    })
    .to_string())
}

fn tmux_list_panes(_args: &[String]) -> Result<String, String> {
    Ok(serde_json::json!({
        "method": "pane.list",
        "params": {}
    })
    .to_string())
}

fn tmux_resize_pane(_args: &[String]) -> Result<String, String> {
    // tmux resize operations: resize-pane -L/-R/-U/-D <amount>
    Ok(serde_json::json!({
        "method": "pane.resize",
        "params": {}
    })
    .to_string())
}

fn parse_flag<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            return iter.next().map(|s| s.as_str());
        }
    }
    None
}
