#[cfg(target_os = "android")]
use std::ffi::CString;
#[cfg(not(target_os = "android"))]
use std::fs;
use std::str;

#[cfg(not(target_os = "android"))]
pub fn read_string(path: &str) -> String {
    fs::read_to_string(format!("./assets/{}", path)).expect(format!("Could not find file: {}!", path).as_str())
}

#[cfg(not(target_os = "android"))]
pub fn read_bytes(path: &str) -> Vec<u8> {
    fs::read(format!("./assets/{}", path)).expect(format!("Could not find file: {}!", path).as_str())
}

#[cfg(target_os = "android")]
pub fn read_bytes(path: &str) -> Vec<u8> {
    let asset_manager = ndk_glue::native_activity().asset_manager();
    let mut opened_asset = asset_manager
        .open(&CString::new(path).unwrap())
        .expect(format!("Could not find file: {}!", path).as_str());

    opened_asset.get_buffer().unwrap().to_vec()
}

#[cfg(target_os = "android")]
pub fn read_string(path: &str) -> String {
    let bytes = read_bytes(path);
    str::from_utf8(bytes.as_slice()).unwrap().to_string()
}
