extern crate libc;
use libc::{c_int, size_t};

#[link(name = "snappy")]
extern "C" {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
    fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: size_t,
        result: *mut size_t,
    ) -> c_int;

    fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t,
    ) -> c_int;
    fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: size_t,
        output: *mut u8,
        output_length: *mut size_t,
    ) -> c_int;
}

/// Safe wrapper around snappy_compress
fn safe_compress(input: &[u8]) -> Vec<u8> {
    let input_len = input.len() as size_t;
    let mut output_len = unsafe { snappy_max_compressed_length(input_len) };
    let mut output = Vec::with_capacity(output_len as usize);
    unsafe {
        let status = snappy_compress(
            input.as_ptr(),
            input_len,
            output.as_mut_ptr(),
            &mut output_len,
        );
        assert_eq!(status, 0);
        assert!(output_len <= output.capacity());
        output.set_len(output_len);
    };
    output
}

/// Safe wrapper around snappy_uncompress
fn safe_uncompress(input: &[u8]) -> Vec<u8> {
    let input_len = input.len() as size_t;
    let mut output_len = 0;
    unsafe {
        let status =
            snappy_uncompressed_length(input.as_ptr(), input.len() as size_t, &mut output_len);
        assert_eq!(status, 0);
    };
    let mut output = Vec::with_capacity(output_len as usize);
    unsafe {
        let status = snappy_uncompress(
            input.as_ptr(),
            input_len,
            output.as_mut_ptr(),
            &mut output_len,
        );
        assert_eq!(status, 0);
        assert!(output_len <= output.capacity());
        output.set_len(output_len);
    };
    output
}

fn main() {
    let text = String::from("Hello, World!");
    let compressed = safe_compress(text.as_bytes());
    let uncompressed = safe_uncompress(&compressed);
    println!("text: {}", text);
    println!("raw: {:?}", text.as_bytes());
    println!("compressed: {:?}", compressed);
    println!("uncompressed: {:?}", uncompressed);
    if uncompressed == text.as_bytes() {
        println!("they match!");
    } else {
        println!("[ERROR] uncompressed text does not match original");
    }
}
