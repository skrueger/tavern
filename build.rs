pub fn main() {
    // This crate gets symbols from layout.ld and kernel.S.
    // If those change, then this crate needs to rebuild.
    println!("cargo:rerun-if-changed=src/layout.ld");
    println!("cargo:rerun-if-changed=src/kernel.S");
}
