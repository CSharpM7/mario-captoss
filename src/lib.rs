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

mod mario;
mod imports;
mod util;
pub mod vars;

#[skyline::main(name = "mario-captoss")]
pub fn main() {
    println!("[smashline2_cappy::main] Loading");
    mario::install();
    println!("[smashline2_cappy::main] Loaded!");
}
