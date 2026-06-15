use std::array;

use jni_sys::{
    jbyte, jbyteArray, jclass, jsize, JNIEnv,
};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_compressBytes<'a>(
    env: *mut JNIEnv,
    _class: jclass,
    bytes: jbyteArray,
) {
    let arrayFuncs = (**env).v1_1;

}