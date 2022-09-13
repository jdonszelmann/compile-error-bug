
pub fn test() {
    // compile error when compiling both test1 and test2
    // compile_error!("test")

    // compile error when compiling both test1 and test2
    // let a: u8 = -5;

    // compile error when compiling test2
    // but *not* when compiling test2
    1u32 << 32;
}
