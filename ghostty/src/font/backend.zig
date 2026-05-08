const std = @import("std");
pub const Backend = enum {
    const WasmTarget = @import("../os/wasm/target.zig").Target;
    freetype, fontconfig_freetype, coretext, web_canvas,
    pub fn default(t: std.Target, w: WasmTarget) Backend { _ = w; _ = t; return .fontconfig_freetype; }
    pub fn hasCoretext(_: Backend) bool { return false; }
    pub fn hasFontconfig(_: Backend) bool { return true; }
    pub fn hasFreetype(_: Backend) bool { return true; }
    pub fn hasHarfbuzz(_: Backend) bool { return true; }
};
