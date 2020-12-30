use jieba_rs::Jieba;

use std::ffi::{CStr, CString};
use std::mem::{forget, drop};
use std::os::raw::c_char;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct JiebaCStrVec {
    pub ptr: *mut *mut c_char,
    pub len: usize,
    pub cap: usize,
}

impl JiebaCStrVec {
    fn from_vec(mut vec: Vec<*mut c_char>) -> Self {
        let len = vec.len();
        let cap = vec.capacity();
        let ptr = vec.as_mut_ptr();
        forget(vec);
        Self { ptr, cap, len }
    }

    unsafe fn into_vec(self) -> Vec<*mut c_char> {
        let JiebaCStrVec { ptr, cap, len } = self;
        Vec::from_raw_parts(ptr, len, cap)
    }
}

#[no_mangle]
pub unsafe extern "C" fn jieba_make_handler() -> *mut Jieba {
    Box::into_raw(Box::new(Jieba::new()))
}

#[no_mangle]
pub unsafe extern "C" fn jieba_destroy_handler(handler: *mut Jieba) {
    drop(Box::from_raw(handler))
}

#[no_mangle]
pub unsafe extern "C" fn jieba_cut(
    handler: *mut Jieba,
    string: *mut c_char
) -> JiebaCStrVec {
    let jb_ref = &*handler;
    // Though spread panic across FFI boundary is UB
    // but we can assume there's always valid UTF-8 for simplicity.
    let string = CStr::from_ptr(string).to_str().unwrap();
    let vec = jb_ref.cut(string, true).into_iter()
    // Assmue we don't have NUL in string
        .map(|x| CString::into_raw(CString::new(x).unwrap()))
        .collect::<Vec<*mut c_char>>();
    JiebaCStrVec::from_vec(vec)
}


#[no_mangle]
pub unsafe extern "C" fn jieba_destroy_cut_result(
    cut_res: JiebaCStrVec
) {
    let vec = JiebaCStrVec::into_vec(cut_res);
    for x in vec {
        drop(CString::from_raw(x));
    }
}
