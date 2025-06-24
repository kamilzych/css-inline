use jni::{
    JNIEnv,
    objects::{JClass, JString},
    sys::jstring,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_CssInline_inline<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    let html: String = env.get_string(&input).unwrap().into();

    let inlined = css_inline::inline(&html).unwrap();

    env.new_string(inlined).unwrap().into_raw()
}
