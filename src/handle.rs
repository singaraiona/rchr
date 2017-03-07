#[inline]
pub fn into_raw<T>(t: &mut T) -> *mut T {
    t as *mut T
}

#[inline]
pub fn from_raw<'a, T: 'a>(t: *mut T) -> &'a mut T {
    unsafe { &mut *t }
}