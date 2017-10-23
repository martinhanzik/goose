pub mod ffi;
pub mod context;
pub mod value;


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    use context::Context;

    #[test]
    fn concat() {
        let context = Context::new();
        let result = CString::new("foo123true").unwrap();
        let ctx = context.get_context();
        let foo = "foo".to_owned();
        context.push(&foo);
        context.push(123);
        context.push(true);
        context.concat(3);
        unsafe {
            assert_eq!(CStr::from_ptr(ffi::duk_get_string(ctx, -1)).to_bytes(), result.to_bytes());
        }
        context.pop();
    }
}