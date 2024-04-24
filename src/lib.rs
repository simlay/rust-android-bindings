use jaffi_support::{
    jni::{objects::JObject, JNIEnv},
    Error,
};

use crate::bindings::{
    AndroidContentContext,
    //AndroidViewView,
    *
};

pub mod bindings {
    #![allow(
        dead_code,
        clippy::unused_unit,
        clippy::needless_lifetimes,
        clippy::let_unit_value,
        clippy::let_and_return
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
}

#[test]
fn simple() {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.expect("Failed to get javavm");
}
