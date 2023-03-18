use std::str;

#[cfg(not(target_os = "android"))]
pub fn read_string(path: &str) -> String {
    std::fs::read_to_string(format!("./assets/{}", path)).unwrap_or_else(|_| panic!("Could not find file: {}!", path))
}

#[cfg(not(target_os = "android"))]
pub fn read_bytes(path: &str) -> Vec<u8> {
    std::fs::read(format!("./assets/{}", path)).unwrap_or_else(|_| panic!("Could not find file: {}!", path))
}

#[cfg(target_os = "android")]
pub fn read_bytes(path: &str) -> Vec<u8> {
    let asset_manager = ndk_glue::native_activity().asset_manager();
    let mut opened_asset = asset_manager
        .open(&std::ffi::CString::new(path).unwrap())
        .unwrap_or_else(|| panic!("Could not find file: {}!", path));

    opened_asset.get_buffer().unwrap().to_vec()
}

#[cfg(target_os = "android")]
pub fn read_string(path: &str) -> String {
    let bytes = read_bytes(path);
    str::from_utf8(bytes.as_slice()).unwrap().to_string()
}
