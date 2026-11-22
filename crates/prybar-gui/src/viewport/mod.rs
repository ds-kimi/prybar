//! 3D model preview. GPUI draws 2D only, so a wgpu renderer on its own
//! thread draws offscreen and the view shows each frame as an image.

pub mod camera;
pub mod mesh;
pub mod render;
mod shaders;
pub mod view;
