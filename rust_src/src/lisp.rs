// //! This module contains Rust definitions whose C equivalents live in
// //! lisp.h.

use libc::{c_void, intptr_t};

use crate::{
    remacs_sys::EmacsInt,
    remacs_sys::{Aligned_Lisp_Subr, Lisp_Subr},
    remacs_sys::{Qnil, Qt, VALMASK},
};

// TODO: tweak Makefile to rebuild C files if this changes.

/// Emacs values are represented as tagged pointers. A few bits are
/// used to represent the type, and the remaining bits are either used
/// to store the value directly (e.g. integers) or the address of a
/// more complex data type (e.g. a cons cell).
///
/// TODO: example representations
///
/// `EmacsInt` represents an integer big enough to hold our tagged
/// pointer representation.
///
/// In Emacs C, this is `EMACS_INT`.
///
/// `EmacsUint` represents the unsigned equivalent of `EmacsInt`.
/// In Emacs C, this is `EMACS_UINT`.
///
/// Their definition are determined in a way consistent with Emacs C.
/// Under casual systems, they're the type isize and usize respectively.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);

impl LispObject {
    pub const fn from_C(n: EmacsInt) -> Self {
        Self(n)
    }

    pub const fn to_C(self) -> EmacsInt {
        self.0
    }
}

impl From<()> for LispObject {
    fn from(_v: ()) -> Self {
        Qnil
    }
}

impl From<LispObject> for bool {
    fn from(o: LispObject) -> Self {
        o.is_not_nil()
    }
}

impl From<bool> for LispObject {
    fn from(v: bool) -> Self {
        if v {
            Qt
        } else {
            Qnil
        }
    }
}

impl LispObject {
    pub fn is_nil(self) -> bool {
        self == Qnil
    }

    pub fn is_not_nil(self) -> bool {
        self != Qnil
    }

    pub fn is_t(self) -> bool {
        self == Qt
    }

    pub fn eq(self, other: impl Into<Self>) -> bool {
        self == other.into()
    }
}

impl LispObject {
    pub fn get_untaggedptr(self) -> *mut c_void {
        (self.to_C() & VALMASK) as intptr_t as *mut c_void
    }
}

// ExternalPtr

#[repr(transparent)]
pub struct ExternalPtr<T>(*mut T);

impl<T> Copy for ExternalPtr<T> {}

// Derive fails for this type so do it manually
impl<T> Clone for ExternalPtr<T> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}

impl<T> ExternalPtr<T> {
    pub const fn new(p: *mut T) -> Self {
        Self(p)
    }
}

pub type LispSubrRef = ExternalPtr<Aligned_Lisp_Subr>;
impl LispSubrRef {
    pub fn as_mut(self) -> *mut Aligned_Lisp_Subr {
        self.0
    }
}
unsafe impl Sync for LispSubrRef {}

macro_rules! export_lisp_fns {
    ($($(#[$($meta:meta),*])* $f:ident),+) => {
	pub fn rust_init_syms() {
	    #[allow(unused_unsafe)] // just in case the block is empty
	    unsafe {
		$(
		    $(#[$($meta),*])* crate::lisp::defsubr(concat_idents!(S, $f).as_mut());
		)+
	    }
	}
    }
}

#[allow(unused_macros)]
macro_rules! protect_statics_from_GC {
    ($($f:ident),+) => {
	pub fn rust_static_syms() {
	    unsafe {
		$(
		    crate::remacs_sys::staticpro(&$f as *const LispObject as *mut LispObject);
		)+
	    }
	}
    }
}

extern "C" {
    pub fn defsubr(sname: *mut Aligned_Lisp_Subr);
}
