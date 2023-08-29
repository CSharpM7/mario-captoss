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
//mod util;
pub mod vars;
mod custom_vars;
//pub mod data;
//use data::gamemode::*;

#[skyline::main(name = "mario-captoss")]
pub fn main() {
    println!("[smashline2_mario::main] Loading");
    custom_vars::install();
    mario::install();
    println!("[smashline2_mario::main] Loaded!");
}
