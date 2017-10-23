#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn duk_create_heap_default() -> *mut duk_context {
    duk_create_heap(None, None, None, std::ptr::null_mut(), None)
}