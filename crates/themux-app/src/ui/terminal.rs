// Ghostty terminal wrapper using libghostty-vt.

use ghostty_sys::{
    ghostty_render_state_colors_get, ghostty_render_state_free,
    ghostty_render_state_new, ghostty_render_state_update, ghostty_terminal_free,
    ghostty_terminal_get, ghostty_terminal_new, ghostty_terminal_resize, ghostty_terminal_vt_write,
    GhosttyRenderState, GhosttyRenderStateColors, GhosttyTerminal,
    GhosttyTerminalOptions, GhosttyResult_GHOSTTY_SUCCESS,
    GhosttyTerminalData_GHOSTTY_TERMINAL_DATA_CURSOR_X,
    GhosttyTerminalData_GHOSTTY_TERMINAL_DATA_CURSOR_Y,
};

use gtk4::prelude::*;
use gtk4::{Label, Widget};

pub struct TerminalWidget {
    terminal: GhosttyTerminal,
    render_state: GhosttyRenderState,
    container: gtk4::Box,
}

impl TerminalWidget {
    pub fn new(cols: u16, rows: u16, cell_width: u32, cell_height: u32) -> Result<Self, &'static str> {
        let options = GhosttyTerminalOptions {
            cols,
            rows,
            max_scrollback: 10000,
        };

        let mut terminal: GhosttyTerminal = std::ptr::null_mut();
        let result = unsafe {
            ghostty_terminal_new(std::ptr::null(), &mut terminal, options)
        };

        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            return Err("Failed to create terminal");
        }

        let result = unsafe {
            ghostty_terminal_resize(terminal, cols, rows, cell_width, cell_height)
        };

        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            unsafe { ghostty_terminal_free(terminal) };
            return Err("Failed to resize terminal");
        }

        let mut render_state: GhosttyRenderState = std::ptr::null_mut();
        let result = unsafe {
            ghostty_render_state_new(std::ptr::null(), &mut render_state)
        };

        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            unsafe { ghostty_terminal_free(terminal) };
            return Err("Failed to create render state");
        }

        let result = unsafe {
            ghostty_render_state_update(render_state, terminal)
        };

        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            unsafe {
                ghostty_render_state_free(render_state);
                ghostty_terminal_free(terminal);
            }
            return Err("Failed to update render state");
        }

        let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        let label = Label::new(Some("Ghostty Terminal"));
        label.add_css_class("terminal-view");
        label.set_vexpand(true);
        label.set_hexpand(true);
        container.append(&label);

        Ok(Self {
            terminal,
            render_state,
            container,
        })
    }

    pub fn widget(&self) -> &Widget {
        self.container.upcast_ref()
    }

    pub fn write(&mut self, data: &[u8]) {
        unsafe {
            ghostty_terminal_vt_write(self.terminal, data.as_ptr(), data.len());
            ghostty_render_state_update(self.render_state, self.terminal);
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16, cell_width: u32, cell_height: u32) {
        unsafe {
            ghostty_terminal_resize(self.terminal, cols, rows, cell_width, cell_height);
            ghostty_render_state_update(self.render_state, self.terminal);
        }
    }

    pub fn cursor_pos(&self) -> Option<(u16, u16)> {
        let mut x: u16 = 0;
        let mut y: u16 = 0;
        let result = unsafe {
            ghostty_terminal_get(
                self.terminal,
                GhosttyTerminalData_GHOSTTY_TERMINAL_DATA_CURSOR_X,
                &mut x as *mut _ as *mut std::ffi::c_void,
            )
        };
        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            return None;
        }
        let result = unsafe {
            ghostty_terminal_get(
                self.terminal,
                GhosttyTerminalData_GHOSTTY_TERMINAL_DATA_CURSOR_Y,
                &mut y as *mut _ as *mut std::ffi::c_void,
            )
        };
        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            return None;
        }
        Some((x, y))
    }

    pub fn get_colors(&self) -> Option<GhosttyRenderStateColors> {
        let mut colors: GhosttyRenderStateColors = unsafe { std::mem::zeroed() };
        colors.size = std::mem::size_of::<GhosttyRenderStateColors>() as _;
        let result = unsafe {
            ghostty_render_state_colors_get(self.render_state, &mut colors)
        };
        if result != GhosttyResult_GHOSTTY_SUCCESS as i32 {
            None
        } else {
            Some(colors)
        }
    }
}

impl Drop for TerminalWidget {
    fn drop(&mut self) {
        unsafe {
            ghostty_render_state_free(self.render_state);
            ghostty_terminal_free(self.terminal);
        }
    }
}