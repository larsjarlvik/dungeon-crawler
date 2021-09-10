mod assets;
mod display;
mod wakelock;

#[cfg(target_os = "android")]
pub use wakelock::aquire_wakelock;

pub use assets::read_bytes;
pub use assets::read_string;
pub use display::get_scale_factor;
