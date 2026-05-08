// themux-core: Core business logic, data models, and state management.
//
// This crate is protocol-agnostic and UI-agnostic. It provides:
// - Workspace/pane/surface/split hierarchy
// - Configuration loading and management
// - Session persistence (snapshot, restore)
// - Split tree layout engine (Bonsplit replacement)
// - Notification store
// - Remote workspace transport model

pub mod config;
pub mod layout;
pub mod notification;
pub mod remote;
pub mod session;
pub mod workspace;
