use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.

use jni::objects::{JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jobject, jstring};

use crate::commons::Barcode;
use anyhow::Result;

fn compute_code_string_from_string<B>(
    env: &mut JNIEnv,
    _class: JClass,
    code: JString,
) -> Result<jstring>
where
    B: Barcode,
    <B as Barcode>::Error: Sync + Send + 'static,
{
    let input: String = env.get_string(&code)?.into();

    let code = B::make_descriptor(&input)?;

    let output = env.new_string(code)?;
    Ok(output.into_raw())
}

fn jni_descriptor_from_string<B>(mut env: JNIEnv, class: JClass, code: JString) -> jstring
where
    B: Barcode,
    <B as Barcode>::Error: Sync + Send + 'static,
{
    match compute_code_string_from_string::<B>(&mut env, class, code) {
        Ok(desc) => return desc,
        Err(e) => {
            env.throw(format!("error creating the code: {}", e))
                .unwrap();
            JObject::null().into_raw()
        }
    }
}

macro_rules! jni_call {
    ($t:ty) => {
        paste::item! {
        #[no_mangle]
        pub extern "system" fn [< Java_net_yageek_codekit_CodeKit_make $t >](
            env: JNIEnv,
            // This is the class that owns our static method. It's not going to be used,
            // but still must be present to match the expected signature of a static
            // native method.
            class: JClass,
            code: JString,
        ) -> jobject {
            jni_descriptor_from_string::<$t>(env, class, code)
        }
        }
    };
}

use crate::{Codabar, Code39, Code93, I2of5, EAN13, EAN8};
jni_call!(EAN8);

jni_call!(EAN13);
jni_call!(Codabar);
jni_call!(Code39);
jni_call!(Code93);
jni_call!(I2of5);
