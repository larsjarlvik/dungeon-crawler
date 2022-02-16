#[cfg(target_os = "android")]
pub fn aquire_wakelock() {
    let native_activity = ndk_glue::native_activity();
    let vm_ptr = native_activity.vm();
    let vm = unsafe { jni::JavaVM::from_raw(vm_ptr) }.unwrap();
    let env = vm.attach_current_thread().unwrap();

    let window = env
        .call_method(native_activity.activity(), "getWindow", "()Landroid/view/Window;", &[])
        .unwrap()
        .l()
        .unwrap();

    const FLAG_KEEP_SCREEN_ON: jni::sys::jint = 128;
    match env.call_method(window, "addFlags", "(I)V", &[FLAG_KEEP_SCREEN_ON.into()]) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to aquire wakelock!");
        }
    }
}
