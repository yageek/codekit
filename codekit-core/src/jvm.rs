use std::ffi::c_void;

use jni::{JNIEnv, JavaVM};

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JValue};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jobject, jstring, JNI_VERSION_1_6};
use log::{debug, trace};

#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(target_os = "android")]
use log::Level;

use crate::commons::Barcode;
use crate::ean::EAN8;
use crate::CodeOptions;

#[cfg(target_os = "android")]
fn setup_android_logger() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("codekit-android"), // logs will show under mytag tag
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    #[cfg(target_os = "android")]
    setup_android_logger();

    debug!("JNI OnLoad !!!");
    let env = vm.get_env().expect("Cannot get reference to the JNIEnv");

    cache::init_cache(&env);
    JNI_VERSION_1_6
}

mod cache {
    use std::sync::Once;

    use jni::{
        objects::{GlobalRef, JMethodID},
        JNIEnv,
    };
    use log::{debug, trace};

    static INIT: Once = Once::new();

    static mut MODEL_CODE_OPTIONS_CLASS: Option<GlobalRef> = None;
    static mut MODEL_CODE_OPTIONS_INIT: Option<JMethodID> = None;

    static mut MODEL_CODE_DESCRIPTOR_CLASS: Option<GlobalRef> = None;
    static mut MODEL_CODE_DESCRIPTOR_INIT: Option<JMethodID> = None;

    fn check_cache_initialized() {
        if !INIT.is_completed() {
            panic!("JNI cache is not initialized")
        }
    }
    pub fn init_cache(env: &JNIEnv) {
        INIT.call_once(|| unsafe { cache_method(env) });
    }

    unsafe fn cache_method(env: &JNIEnv) {
        debug!("Init cache");

        trace!("Getting code descriptor gloabal ref...");
        MODEL_CODE_DESCRIPTOR_CLASS = get_class(&env, "net/yageek/codekit/CodeDescriptor");

        trace!("Getting code descriptor init method...");
        MODEL_CODE_DESCRIPTOR_INIT = get_method_id(
            &env,
            "net/yageek/codekit/CodeDescriptor",
            "<init>",
            "(Lnet/yageek/codekit/CodeOptions;[B)V",
        );

        debug!("Init cache ended");
    }

    /// Produces `JMethodID` for a particular method dealing with its lifetime.
    ///
    /// Always returns `Some(method_id)`, panics if method not found.
    fn get_method_id(
        env: &JNIEnv,
        class: &str,
        name: &str,
        sig: &str,
    ) -> Option<JMethodID<'static>> {
        let method_id = env
            .get_method_id(class, name, sig)
            // we need this line to erase lifetime in order to save underlying raw pointer in static
            .map(|mid| mid.into_inner().into())
            .unwrap_or_else(|_| {
                panic!(
                    "Method {} with signature {} of class {} not found",
                    name, sig, class
                )
            });
        Some(method_id)
    }

    /// Returns cached class reference.
    ///
    /// Always returns Some(class_ref), panics if class not found.
    fn get_class(env: &JNIEnv, class: &str) -> Option<GlobalRef> {
        let class = env
            .find_class(class)
            .unwrap_or_else(|_| panic!("Class {} not found", class));
        Some(env.new_global_ref(class).unwrap())
    }

    pub fn code_descriptor_init() -> JMethodID<'static> {
        check_cache_initialized();
        unsafe { MODEL_CODE_DESCRIPTOR_INIT.unwrap() }
    }

    pub fn code_descriptor_class() -> GlobalRef {
        check_cache_initialized();
        unsafe { MODEL_CODE_DESCRIPTOR_CLASS.clone().unwrap() }
    }
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
    code: JString,
    options: jobject,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(code)
        .expect("Couldn't get java string!")
        .into();

    // Let's trace the input element
    trace!("Input element: {}", input);

    let quiet_space: u16 = env
        .call_method(options, "getQuietSpace", "()I", &[])
        .expect("valid value")
        .i()
        .unwrap()
        .try_into()
        .ok()
        .unwrap_or(7);
    trace!("Quiet space element: {}", quiet_space);

    let code_height = env
        .call_method(options, "getCodeHeight", "()I", &[])
        .expect("valid value")
        .try_into()
        .ok()
        .unwrap_or(50);
    trace!("Codeheight space element: {}", code_height);

    let border_width = env
        .call_method(options, "getBorderWidth", "()I", &[])
        .expect("valid value")
        .i()
        .unwrap()
        .try_into()
        .ok()
        .unwrap_or(0);
    trace!("border space element: {}", border_width);

    let roptions = CodeOptions {
        quiet_space,
        code_height,
        border_width,
    };
    trace!("Descriptor creation");

    let descriptor = EAN8::make_descriptor(&input, roptions).expect("valid code");
    trace!("Descriptor converted");

    let bars: Vec<_> = descriptor
        .get_bars()
        .into_iter()
        .map(|value| value as i8)
        .collect();

    trace!("Creating buffer converted");
    let buffer = env.new_byte_array(bars.len() as i32).expect("valid array");
    env.set_byte_array_region(buffer, 0, &bars[..]).unwrap();

    // Now that we have the descriptor
    // We can convert back to java
    let output = env
        .new_object_unchecked(
            &cache::code_descriptor_class(),
            cache::code_descriptor_init(),
            &[
                JValue::Object(options.into()),
                JValue::Object(buffer.into()),
            ],
        )
        .expect("valid object");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}
