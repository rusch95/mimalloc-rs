/*
uintptr_t _mi_align_up(uintptr_t sz, size_t alignment) {
  uintptr_t x = (sz / alignment) * alignment;
  if (x < sz) x += alignment;
  if (x < sz) return 0; // overflow
  return x;
}

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    println!("Hello, World!, {}", input);
    input * 2
}
*/
