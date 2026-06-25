use std::{
    slice,
    mem,
};

use jni_sys::{JNIEnv, jbyteArray, jclass};
use libdeflater::{CompressionLvl, Compressor, DecompressionError, Decompressor};

pub struct JByteArr {
    env: *mut JNIEnv,
    arr: jbyteArray,
}

impl From<JByteArr> for Vec<u8> {
    fn from(value: JByteArr) -> Self {
        let env = value.env;
        let bytes = value.arr;
        let jni_1_1 = unsafe { (**env).v1_1 };
        let (slice, buff) = unsafe {
            let arr_size = (jni_1_1.GetArrayLength)(env, bytes);
            let buff = libc::malloc(mem::size_of::<i8>() * arr_size as usize) as *mut i8;
            (jni_1_1.GetByteArrayRegion)(env, bytes, 0, arr_size, buff);
            let slice = slice::from_raw_parts(buff as *mut u8, arr_size as usize);
            (slice, buff)
        };
        let vec = slice.to_vec();
        unsafe {
            libc::free(buff as *mut libc::c_void);
        };
        vec
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_net_izom_libdeflater_Binding_compressBytes(
    env: *mut JNIEnv,
    _: jclass,
    bytes: jbyteArray,
) -> jbyteArray {
    let jni_1_1 = unsafe { (**env).v1_1 };
    let vec: Vec<u8> = JByteArr { env, arr: bytes }.into();

    let mut compressor = Compressor::new(CompressionLvl::default());
    let mut buff = Vec::new();
    buff.resize(compressor.deflate_compress_bound(vec.len()), 0);
    let compressed_size = compressor
        .deflate_compress(vec.as_slice(), &mut buff)
        .unwrap();

    buff.truncate(compressed_size);
    let buff: Vec<i8> = buff.iter().map(|&b| b as i8).collect();

    unsafe {
        let array = (jni_1_1.NewByteArray)(env, buff.len() as i32);
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
        let buff: *mut i8 = libc::malloc(mem::size_of::<i32>() * size) as *mut i8;

        if buff.is_null() {
            return (array_funcs.NewByteArray)(env, 0);
        }

        (array_funcs.GetByteArrayRegion)(env, bytes, 0, size as i32, buff);

        let slice = slice::from_raw_parts(buff as *mut u8, size);

        libc::free(buff as *mut libc::c_void);
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
            }
            Err(DecompressionError::BadData) => {
                println!();
                println!();
                buff = vec![];
                break;
            }
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
