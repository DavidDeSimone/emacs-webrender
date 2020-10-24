use remacs_macros::lisp_fn;
use lazy_static::lazy_static;

#[lisp_fn]
pub fn eptn() -> crate::lisp::LispObject {
    crate::lisp::LispObject::from(true)
}

include!(concat!(env!("OUT_DIR"), "/example_exports.rs"));
