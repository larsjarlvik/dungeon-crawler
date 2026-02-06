mod assets;
mod io;

#[cfg(target_os = "android")]
mod android;

pub use io::exists;
pub use io::read_file;
pub use io::write_file;

pub use assets::read_bytes;
pub use assets::read_string;

#[cfg(target_os = "android")]
pub use android::set_android_app;
#[cfg(target_os = "android")]
pub use android::with_android_app;
