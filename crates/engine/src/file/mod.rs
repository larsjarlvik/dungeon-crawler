mod assets;
mod file;

pub use file::exists;
pub use file::read_file;
pub use file::write_file;

pub use assets::read_bytes;
pub use assets::read_string;
