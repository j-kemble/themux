// themux-socket: Unix domain socket server implementing cmux's V2 JSON-RPC protocol.
//
// Provides:
// - Socket listener with password-based authentication
// - V2 JSON-RPC method dispatch
// - Event bus (streaming events)
// - TCP relay support for remote workspaces
//
// Protocol compatibility with macOS cmux enables reuse of:
// - The Python test suite (tests_v2/)
// - The cmux CLI (if pointed at themux socket)
// - Agent hooks that speak the V2 protocol

pub mod event_bus;
pub mod protocol;
pub mod server;
