mod vibrate;
mod wakelock;

pub use vibrate::vibrate;

#[cfg(target_os = "android")]
pub use wakelock::aquire_wakelock;
