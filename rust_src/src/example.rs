use lazy_static::lazy_static;
use remacs_macros::lisp_fn;

#[lisp_fn]
pub fn eptn(t: crate::lisp::LispObject) -> crate::lisp::LispObject {
    crate::lisp::LispObject::from(true)
}

include!(concat!(env!("OUT_DIR"), "/example_exports.rs"));
