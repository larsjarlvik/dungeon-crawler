use std::{fs, io};

fn get_path(name: &str) -> String {
    #[cfg(target_os = "android")]
    {
        let data_path = super::android::internal_data_path();
        format!("{}/{name}", data_path.to_str().unwrap())
    }

    #[cfg(not(target_os = "android"))]
    format!("./{}", name)
}

pub fn write_file(name: &str, contents: &str) {
    fs::write(get_path(name), contents).unwrap();
}

pub fn read_file(name: &str) -> Result<String, io::Error> {
    fs::read_to_string(get_path(name))
}

pub fn exists(path: &str) -> bool {
    #[cfg(target_os = "android")]
    {
        super::android::with_asset_manager(|asset_manager| {
            asset_manager.open(&std::ffi::CString::new(path).unwrap()).is_some()
        })
    }

    #[cfg(not(target_os = "android"))]
    std::path::Path::new(format!("./assets/{}", path).as_str()).exists()
}
