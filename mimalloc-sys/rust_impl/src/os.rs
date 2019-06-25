use libc::*;

#[no_mangle]
pub extern "C" fn _mi_align_up_rs(sz: uintptr_t, alignment: size_t) -> uintptr_t {
    let x = (sz / alignment) * alignment;
    if x < sz { 
        let (x, overflowed) = x.overflowing_add(alignment); 
        if overflowed {
            0
        } else {
            x
        }
    } else {
        x
    }
}
