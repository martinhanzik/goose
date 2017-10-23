extern crate duktape_sys as ffi;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn sanity() {
        let foo = CString::new("foo").unwrap();
        let result = CString::new("foo123true").unwrap();
        unsafe {
            let ctx = ffi::duk_create_heap_default();

            ffi::duk_push_string(ctx, foo.as_ptr());
            ffi::duk_push_int(ctx, 123);
            ffi::duk_push_true(ctx);
            ffi::duk_concat(ctx, 3);
            assert_eq!(CStr::from_ptr(ffi::duk_get_string(ctx, -1)).to_bytes(), result.to_bytes());
            ffi::duk_pop(ctx);

            ffi::duk_destroy_heap(ctx);
        }
    }
}