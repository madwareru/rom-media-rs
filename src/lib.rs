pub mod audio;
pub mod video;
pub mod windowing;
pub mod gl { include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs")); }