// Session persistence: snapshot, serialize, auto-save, restore.
//
// Compatible with cmux's AppSessionSnapshot JSON format.

pub mod persistence;
pub mod snapshot;

pub use snapshot::AppSessionSnapshot;
pub use persistence::SessionPersistence;
