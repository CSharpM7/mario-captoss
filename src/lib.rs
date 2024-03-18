#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    hash40,
    lua2cpp::*,
    phx::*
};
use smashline::*;

mod imports;
mod util;
pub mod vars;
mod installer;

mod mario;

#[skyline::main(name = "mario-captoss")]
pub fn main() {
    println!("[smashline2_cappy::main] Loading");
    #[cfg(not(feature = "dev"))]
    installer::install();
    #[cfg(feature = "dev")]
    installer::smashline_install();
    println!("[smashline2_cappy::main] Loaded!");
}
