use std::ffi::c_void;

use jni::{JNIEnv, JavaVM};

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{GlobalRef, JClass, JMethodID, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jstring, JNI_VERSION_1_6};
use log::trace;

#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(target_os = "android")]
use log::Level;

static mut MODEL_CODE_OPTIONS_CLASS: Option<GlobalRef> = None;
static mut MODEL_CODE_OPTIONS_INIT: Option<JMethodID> = None;

static mut MODEL_CODE_DESCRIPTOR_CLASS: Option<GlobalRef> = None;
static mut MODEL_CODE_DESCRIPTOR_INIT: Option<JMethodID> = None;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    trace!("JNI OnLoad !!!");
    let env = vm.get_env().expect("Cannot get reference to the JNIEnv");

    #[cfg(target_os = "android")]
    setup_android_logger();

    init_cache(&env);
    JNI_VERSION_1_6
}
fn init_cache(env: &JNIEnv) {}
#[cfg(target_os = "android")]
fn setup_android_logger() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("codekit-android"), // logs will show under mytag tag
    );
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_net_yageek_codekit_CodeKit_makeEAN8(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}
