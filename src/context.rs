extern crate num;

use ffi;
use value::Value;

pub struct Context {
    pub(crate) context: *mut ffi::duk_context
}


impl Context {
    pub fn new() -> Context {
        Context {
            context: unsafe { ffi::duk_create_heap_default() },
        }
    }

    pub fn get_context(&self) -> *mut ffi::duk_context {
        self.context
    }

    pub fn push<T>(&self, value: T)
        where T: Value {
        value.push(self)
    }

    pub fn pop(&self) {
        unsafe { ffi::duk_pop(self.context) }
    }

    pub fn concat(&self, count: i32) {
        unsafe { ffi::duk_concat(self.context, count) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::duk_destroy_heap(self.context) }
    }
}