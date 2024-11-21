mod acmd;
mod agent;
mod status;
mod frame;
mod other_fighters;

use smash::lib::lua_const::WEAPON_KIND_KOOPAJR_CANNONBALL;
use smashline;
pub const HAVE: bool = false;
pub const FORCE_FLY: bool = true;
pub const SHOOT: bool = false;

pub fn install() {
    acmd::install();
    status::install();
    agent::install();
    frame::install();
    other_fighters::install();

    smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "mario", "captoss",false);
}