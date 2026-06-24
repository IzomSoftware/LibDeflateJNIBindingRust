use std::slice;

use jni_sys::{JNIEnv, jbyteArray, jclass};
use libdeflater::{CompressionLvl, Compressor, DecompressionError, Decompressor};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_compressBytes(
    env: *mut JNIEnv,
    _: jclass,
    bytes: jbyteArray,
) -> jbyteArray {
    let jni_1_1 = unsafe { (**env).v1_1 };
    let size = unsafe { (jni_1_1.GetArrayLength)(env, bytes) } as usize;

    let slice = unsafe {
        let buff: *mut i8 = libc::malloc(std::mem::size_of::<i32>() * size) as *mut i8;

        if buff.is_null() {
            return (jni_1_1.NewByteArray)(env, 0);
        }
        (jni_1_1.GetByteArrayRegion)(env, bytes, 0, size as i32, buff);

        let slice = slice::from_raw_parts(buff as *mut u8, size as usize);

        libc::free(buff as *mut libc::c_void);
        slice
    };

    let mut compressor = Compressor::new(CompressionLvl::default());
    let (compressed_size, buff): (usize, Vec<i8>) = {
        let mut buff = Vec::new();
        buff.resize(compressor.deflate_compress_bound(slice.len()), 0);
        let compressed_size = compressor.deflate_compress(slice, &mut buff).unwrap();
        buff.truncate(compressed_size);
        (compressed_size, buff.iter().map(|&b| b as i8).collect())
    };

    unsafe {
        let array = (jni_1_1.NewByteArray)(env, compressed_size as i32);
        (jni_1_1.SetByteArrayRegion)(env, array, 0, buff.len() as i32, buff.as_ptr());
        array
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_decompressBytes(
    env: *mut JNIEnv,
    _: jclass,
    bytes: jbyteArray,
) -> jbyteArray {
    let array_funcs = unsafe { (**env).v1_1 };
    let size = unsafe { (array_funcs.GetArrayLength)(env, bytes) as usize };
    let slice = unsafe {
        let buff: *mut i8 = libc::malloc(std::mem::size_of::<i32>() * size) as *mut i8;

        if buff.is_null() {
            return (array_funcs.NewByteArray)(env, 0);
        }

        (array_funcs.GetByteArrayRegion)(env, bytes, 0, size as i32, buff);

        let slice = slice::from_raw_parts(buff as *mut u8, size);
        slice
    };

    let mut decompressor = Decompressor::new();
    let mut buff = vec![];
    let mut size = slice.len() * 4;
    loop {
        match decompressor.deflate_decompress(slice, &mut buff) {
            Ok(size) => {
                buff.truncate(size);
                break;
            }
            Err(DecompressionError::InsufficientSpace) => {
                size *= 2;
                buff.resize(size, 0);
            },
            _ => buff = vec![],
        }
    }
    let decomkpressed: Vec<i8> = buff.iter().map(|&b| b as i8).collect();
    unsafe {
        let array = (array_funcs.NewByteArray)(env, decomkpressed.len() as i32);
        (array_funcs.SetByteArrayRegion)(
            env,
            array,
            0,
            decomkpressed.len() as i32,
            decomkpressed.as_ptr(),
        );
        array
    }
}
