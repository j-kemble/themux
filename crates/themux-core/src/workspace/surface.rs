// Surface: type alias for Panel. Kept for protocol compatibility.
//
// In the cmux protocol, "surface" and "panel" are used interchangeably.
// Here we use Panel as the canonical name, with Surface as an alias.

pub use super::pane::Panel as Surface;
pub use super::pane::PanelType as SurfaceType;
