use ffi;
use std::ffi::CString;
use context::Context;

pub trait Value {
    fn push(&self, context: &Context);
}

macro_rules! impl_for {
    ($T:ty, $U:ty, $func:ident) => (
        impl Value for $T {
            fn push(&self, context: &Context) {
                unsafe { ffi::$func(context.context, *self as $U) }
            }
        }
    );
    ($T:ty, $func:ident) => (impl_for!($T, $T, $func))

}
impl_for!(isize, f64, duk_push_number);
impl_for!(i8, f64, duk_push_number);
impl_for!(i16, f64, duk_push_number);
impl_for!(i32, f64, duk_push_number);
impl_for!(usize, f64, duk_push_number);
impl_for!(u8, f64, duk_push_number);
impl_for!(u16, f64, duk_push_number);
impl_for!(u32, f64, duk_push_number);
impl_for!(bool, u32, duk_push_boolean);

impl Value for String {
    fn push(&self, context: &Context) {
        let data = CString::new(self.as_bytes()).unwrap();
        let ptr = data.as_ptr();
        unsafe { ffi::duk_push_string(context.context, ptr); }
    }
}

impl<'a> Value for &'a String {
    fn push(&self, context: &Context) {
        let data = self.as_ptr() as *const i8;
        unsafe { ffi::duk_push_string(context.context, data); }
    }
}

impl<'a> Value for &'a str {
    fn push(&self, context: &Context) {
        let data = self.as_ptr() as *const i8;
        unsafe { ffi::duk_push_string(context.context, data); }
    }
}