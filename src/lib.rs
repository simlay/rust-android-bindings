use jaffi_support::{
    jni::{objects::JObject, JNIEnv},
    Error,
};

pub use crate::bindings::{
    AndroidAnimationStateListAnimator,
    //AndroidWidgetEditText,
    //AndroidContentContext,
    //AndroidViewView,
    *
};

mod bindings {
    #![allow(
        dead_code,
        clippy::unused_unit,
        clippy::needless_lifetimes,
        clippy::let_unit_value,
        clippy::let_and_return
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
}
