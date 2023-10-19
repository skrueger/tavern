pub(crate) struct Uart0;

pub(crate) static mut UART0: Uart0 = Uart0;

impl core::fmt::Write for Uart0 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart0_write_str(s);
        Ok(())
    }
}

#[no_mangle]
#[inline]
pub extern "C" fn uart0_write_char(c: u8) {
    unsafe {
        core::ptr::write_volatile(0x3F20_1000 as *mut u8, c);
    }
}

pub(crate) fn uart0_write_str(str: &str) {
    for c in str.bytes() {
        uart0_write_char(c)
    }
}
