use std::io::{Write, stdout};

use jni_sys::{JNIEnv, jbyteArray, jclass};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_compressBytes<'a>(
    env: *mut JNIEnv,
    _class: jclass,
    bytes: jbyteArray,
) {
    let array_funcs = unsafe { (**env).v1_1 };
    let size = unsafe { (array_funcs.GetArrayLength)(env, bytes) } as usize;
    let buff: *mut i8 = unsafe { libc::malloc(std::mem::size_of::<i32>() * size) as *mut i8 };
    if buff.is_null() {
        return;
    }
    unsafe { (array_funcs.GetByteArrayRegion)(env, bytes, 0, size as i32, buff) }
    let slice = unsafe { std::slice::from_raw_parts(buff, size as usize) };
    let slice = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, size as usize) };
    let slice = [slice, b"\n"].concat();
    let slice = slice.as_slice();
    stdout().write_all(slice).unwrap();
    unsafe { libc::free(buff as *mut libc::c_void) };
}
