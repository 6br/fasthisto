#![feature(rustc_private)]
#[macro_use]
extern crate log;

extern crate libc;

extern crate serde_json;
extern crate bitpacking;
extern crate regex;
use deflate::deflate_bytes;
extern crate bam;
use std::ffi::{ CStr };
use std::io;
use bam::RecordWriter;

#[no_mangle]
pub mod range;

use bitpacking::{BitPacker4x, BitPacker};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use range::{Region};
use std::io::prelude::*;
use std::slice;
use numpress::DEFAULT_SCALING;
use numpress::numpress_compress;

use libc::{ c_void, c_char, size_t };

/*
  TEST CASES
0. 普通に呼べるか? -> hello_rust()
1. Rust の構造体を他から呼び出しできるか？ -> decrement_start()
2. flate2 を外部から呼び出しできるか？ -> compress_bytes()
3. bigbed をパースできるか？ -> load_bigbed()
4. Pfor を外部呼び出しで動作させられるか？ ->  bit_packing()

*/

#[no_mangle]
#[repr(C)]
pub struct RustObject {
    a: i32,
    // Other members...
}


#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn bit_packing(ptr: *const u32, len: usize) -> *const u8 {
    let data = unsafe { slice::from_raw_parts(ptr, len) };
    let mut original = vec![0u32; data.len() as usize];
    original.copy_from_slice(&data);

    let bitpacker = BitPacker4x::new();
    let num_bits: u8 = bitpacker.num_bits(&original);

    let mut compressed = vec![0u8; 4 * BitPacker4x::BLOCK_LEN];
    let _compressed_len = bitpacker.compress(&original, &mut compressed[..], num_bits);

    return compressed.as_ptr()
}

#[no_mangle]
pub extern "C" fn deflate(data: [u8;16]) -> *const u8 {
    let compressed = deflate_bytes(&data);
    return compressed.as_ptr()
}

#[no_mangle]
pub extern "C" fn float_compression(data: [f64;4]) -> Option<*const u8> {
    //let floats: Vec<f64> = //vec![100., 101., 102., 103.];
    let mut original = vec![0f64; data.len() as usize];
    original.copy_from_slice(&data);
    let compressed = numpress_compress(&original, DEFAULT_SCALING).map(|t| t.as_ptr()).ok();
/*    return Some(compressed.as_ptr());*/
    return compressed;
}

#[no_mangle]
pub extern "C" fn decrement_start(mut region: Region) -> Region {
    region.start_minus();
    return region
}

#[no_mangle]
pub extern "C" fn bam(c_buf: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    let mut reader = bam::IndexedReader::from_path(str_buf).unwrap();
    let output = io::BufWriter::new(io::stdout());
    let mut writer = bam::SamWriter::build()
        .write_header(false)
        .from_stream(output, reader.header().clone()).unwrap();

    for record in reader.fetch(&bam::Region::new(2, 600_000, 700_000)).unwrap() {
        let record = record.unwrap();
        writer.write(&record).unwrap();
    }
}

/*
#[no_mangle]
pub extern fn load_bigbed(path: String, region: Region) -> Vec<Feature> {
    return libbigbed(
        path,
        &region, 
        "".to_owned(),
    )
}
*/

#[no_mangle]
pub extern "C" fn compress_bytes(words: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(words)?;
    let compressed_bytes = e.finish();
    return compressed_bytes
}

#[no_mangle]
pub extern "C" fn compress_bytes_extern(words: [u8;8]) -> Option<*const u8> {
    return compress_bytes(&words).map(|t| t.as_ptr()).ok()
}

#[no_mangle]
pub extern "C" fn compress_bytes_extern_unwrap(words: [u8;8]) -> *const u8 {
    return compress_bytes(&words).map(|t| t.as_ptr()).ok().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn compress_bytes_test() {
        println!("{:?}", compress_bytes(&[1,2,3,4,5,6,7,8]));
        println!("{:?}", compress_bytes_extern([1,2,3,4,5,6,7,8]));
        unsafe {println!("{:?}", *(compress_bytes_extern([1,2,3,4,5,6,7,8])).unwrap()); }
        assert_eq!(2 + 2, 4);
    }
}
