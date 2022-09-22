use std::{fs, io};

fn get_path(name: &str) -> String {
    #[cfg(target_os = "android")]
    {
        let native_activity = ndk_glue::native_activity();
        return format!("{}/{name}", native_activity.internal_data_path().to_str().unwrap());
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
