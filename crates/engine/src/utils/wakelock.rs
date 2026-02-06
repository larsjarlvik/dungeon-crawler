#[cfg(target_os = "android")]
pub fn aquire_wakelock(app: &android_activity::AndroidApp) {
    use jni::objects::JObject;

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) }.expect("Failed to access Java VM!");
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as *mut _) };
    let mut env = vm.attach_current_thread().expect("Failed to attach current thread!");

    let window = env
        .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])
        .unwrap()
        .l()
        .unwrap();

    const FLAG_KEEP_SCREEN_ON: jni::sys::jint = 128;
    match env.call_method(&window, "addFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()]) {
        Ok(_) => {}
        Err(_) => {
            log::error!("Failed to aquire wakelock!");
        }
    }
}
