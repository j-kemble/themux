const std = @import("std");

pub const Target = enum(u1) {
    browser,

    pub fn current(comptime _: type) Target {
        return .browser;
    }
};
