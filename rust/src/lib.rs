use jni_sys::{JNIEnv, jbyteArray, jclass};
use libdeflater::{CompressionLvl, Compressor};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_compressBytes(
    env: *mut JNIEnv,
    _class: jclass,
    bytes: jbyteArray,
) -> jbyteArray {
    let array_funcs = unsafe { (**env).v1_1 };
    let size = unsafe { (array_funcs.GetArrayLength)(env, bytes) } as usize;
    let buff: *mut i8 = unsafe { libc::malloc(std::mem::size_of::<i32>() * size) as *mut i8 };
    if buff.is_null() {
        return unsafe { (array_funcs.NewByteArray)(env, 0) };
    }
    unsafe { (array_funcs.GetByteArrayRegion)(env, bytes, 0, size as i32, buff) }
    let slice = unsafe { std::slice::from_raw_parts(buff, size as usize) };
    unsafe { libc::free(buff as *mut libc::c_void) };
    let slice = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, size as usize) };
    let mut compressor = Compressor::new(CompressionLvl::default());
    let mut buff = Vec::new();
    buff.resize(compressor.deflate_compress_bound(slice.len()), 0);
    let compressed_size = compressor.deflate_compress(slice, &mut buff).unwrap();
    buff.truncate(compressed_size);
    let buff: Vec<i8> = buff.iter().map(|&b| b as i8).collect();
    let len = buff.len();
    let buff = buff.as_ptr();
    let array = unsafe { (array_funcs.NewByteArray)(env, compressed_size as i32) };
    unsafe { (array_funcs.SetByteArrayRegion)(env, array, 0, len as i32, buff) };
    return array;
}
