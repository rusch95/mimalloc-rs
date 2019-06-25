use libc::*;

#[no_mangle]
pub extern "C" fn _mi_align_up_rs(sz: uintptr_t, alignment: size_t) -> uintptr_t {
    let mut x = sz.wrapping_div(alignment).wrapping_mul(alignment);
    if x < sz { x = x.wrapping_add(alignment) };
    if x < sz { 0 } else { x }
}
