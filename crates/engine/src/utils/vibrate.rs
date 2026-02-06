#[cfg(target_os = "android")]
fn vibrate_android(duration: f32) -> Result<(), Box<dyn std::error::Error>> {
    use crate::file::with_android_app;
    use jni::objects::JObject;

    with_android_app(|app| {
        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) }?;
        let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as *mut _) };
        let mut env = vm.attach_current_thread().expect("Could not attach to thread!");
        let class_ctxt = env.find_class("android/content/Context").expect("Could not get context!");

        let vibrator_manager = env.get_static_field(class_ctxt, "VIBRATOR_MANAGER_SERVICE", "Ljava/lang/String;")?;

        let vibrator_service = env
            .call_method(
                &activity,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[(&vibrator_manager).into()],
            )?
            .l()?;

        let vibration_effect = env.find_class("android/os/VibrationEffect")?;

        let length: jni::sys::jlong = duration as i64;
        const AMPLITUDE: jni::sys::jint = 255;
        let effect = env
            .call_static_method(
                vibration_effect,
                "createOneShot",
                "(JI)Landroid/os/VibrationEffect;",
                &[length.into(), AMPLITUDE.into()],
            )?
            .l()?;

        let vibrator = env
            .call_method(&vibrator_service, "getDefaultVibrator", "()Landroid/os/Vibrator;", &[])?
            .l()?;

        env.call_method(&vibrator, "vibrate", "(Landroid/os/VibrationEffect;)V", &[(&effect).into()])?;

        Ok(())
    })
}

#[allow(unused_variables)]
pub fn vibrate(duration: f32) {
    #[cfg(target_os = "android")]
    match vibrate_android(duration) {
        Ok(_) => {}
        Err(_) => {
            log::error!("Failed to vibrate!");
        }
    }
}
