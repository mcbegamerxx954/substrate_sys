use libc::{c_char, c_void, size_t};
use objc_sys::{objc_class, IMP, SEL};
// Link to cydia substrate
type MSImagePtr = *mut c_void;
type Class = *const objc_class;
#[link(name = "CydiaSubstrate", kind = "framework")]
extern "C" {
    pub fn MSMapImage(file: *const c_char) -> MSImagePtr;
    pub fn MSCloseImage(imgref: MSImagePtr);
    pub fn MSGetImageByName(name: *const c_char) -> MSImagePtr;
    pub fn MSFindSymbol(imgref: MSImagePtr, symname: *const c_char) -> *mut c_void;
    pub fn MSFindAddress(imgref: MSImagePtr, ptr: *mut *mut c_void) -> *mut c_char;
    pub fn MSHookFunction(orig: *mut c_void, hook: *mut c_void, result: *mut *mut c_void);
    pub fn MSHookMemory(target: *mut c_void, data: *const c_void, size: size_t);
    // old arg is not *mut IMP because thats very hard to use safely
    pub fn MSHookMessageEx(class: Class, sel: SEL, imp: IMP, old: *mut *mut c_void);
    pub fn MSHookClassPair(class: Class, hook: Class, old: Class);
}
