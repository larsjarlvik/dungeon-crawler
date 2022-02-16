mod assets;
mod fbm;
mod file;
mod interpolated_value;
mod wakelock;

#[cfg(target_os = "android")]
pub use wakelock::aquire_wakelock;

pub use file::read_file;
pub use file::write_file;

pub use assets::read_bytes;
pub use assets::read_string;
pub use fbm::fbm;
pub use interpolated_value::Interpolate;
pub use interpolated_value::InterpolatedValue;
