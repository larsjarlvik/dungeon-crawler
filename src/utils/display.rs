pub fn get_scale_factor(window: &winit::window::Window) -> f32 {
    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    let mut scale_factor = window.scale_factor() as f32;

    #[cfg(target_os = "android")]
    {
        let am = ndk_glue::native_activity().asset_manager();
        let config = ndk::configuration::Configuration::from_asset_manager(&am);
        scale_factor = config.density().map(|dpi| dpi as f32 / 160.0).unwrap_or(1.0)
    }

    scale_factor
}
