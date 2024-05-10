use android_activity::AndroidApp;
use jni::objects::{JObject, JValue};
use log::info;
use android_bindings::{
    AndroidWidgetEditText,
    AndroidContentContext,
    AndroidWidgetTextView,

};


/// A minimal example of how to use `ndk_context` to get a `JavaVM` + `Context and make a JNI call
fn ndk_context_jni_test() -> Result<(), Box<dyn std::error::Error>> {
    // Get a VM for executing JNI calls
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let env = vm.attach_current_thread()?;
    let context = AndroidContentContext::from(context);

    let text_editor = AndroidWidgetEditText::new_1android_widget_edit_text_landroid_content_context_2(
        *env,
        context,
    );


    // Since we aren't making JNI calls within the implementation of a native call from the JavaVM
    // we wrap the reference in an `AutoLocal` to make sure it will be deleted.
    let _int_ref = env.auto_local(
        env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(42)])
            .unwrap(),
    );

    Ok(())
}

#[no_mangle]
fn android_main(app: AndroidApp) {
    let winow = app.native_window();

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    info!("before hello world");
    println!("before hello world");

    ndk_context_jni_test().unwrap();

    info!("after hello world");
    println!("after hello world");
}
