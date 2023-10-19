/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
#[allow(dead_code)]
pub fn align_down(addr: usize, align: usize) -> usize {
    if !is_power_of_two(align) {
        panic!("{} is not a power of 2", align);
    } else {
        let remainder = addr % align;
        addr - remainder
    }
}

pub fn is_power_of_two(x: usize) -> bool {
    (x != 0) && ((x & (x - 1)) == 0)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    if !is_power_of_two(align) {
        panic!("{} is not a power of 2", align);
    } else {
        let remainder = addr % align;
        if remainder == 0 {
            addr
        } else {
            addr + (align - remainder)
        }
    }
}
