//! Android-specific file access utilities using android-activity

#[cfg(target_os = "android")]
use std::sync::OnceLock;

#[cfg(target_os = "android")]
static ANDROID_APP: OnceLock<android_activity::AndroidApp> = OnceLock::new();

#[cfg(target_os = "android")]
pub fn set_android_app(app: android_activity::AndroidApp) {
    ANDROID_APP.set(app).expect("AndroidApp already initialized!");
}

#[cfg(target_os = "android")]
pub fn with_android_app<F, R>(f: F) -> R
where
    F: FnOnce(&android_activity::AndroidApp) -> R,
{
    let app = ANDROID_APP.get().expect("AndroidApp not initialized! Call set_android_app first.");
    f(app)
}

#[cfg(target_os = "android")]
pub fn with_asset_manager<F, R>(f: F) -> R
where
    F: FnOnce(&ndk::asset::AssetManager) -> R,
{
    let app = ANDROID_APP.get().expect("AndroidApp not initialized! Call set_android_app first.");
    let asset_manager = app.asset_manager();
    f(&asset_manager)
}

#[cfg(target_os = "android")]
pub fn internal_data_path() -> std::path::PathBuf {
    let app = ANDROID_APP.get().expect("AndroidApp not initialized!");
    app.internal_data_path().expect("No internal data path").to_path_buf()
}
