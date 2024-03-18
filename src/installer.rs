
#[no_mangle]
pub fn smashline_install() {
    install();
}

pub fn install() {
    crate::mario::install();
}