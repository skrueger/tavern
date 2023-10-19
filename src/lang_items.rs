//! lang_items.rs holds rust's "lang items".
//!
//! For more information about rust's lang items see:
//! - https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html
//! - https://rustc-dev-guide.rust-lang.org/lang-items.html

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::kprintln!("Kernel Panic!");
    crate::kprintln!("{info}");

    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {
    crate::kprintln!("eh_personality enter");
}
