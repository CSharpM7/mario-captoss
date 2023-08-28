mod acmd;
mod agent;
mod status;
mod frame;
use smashline;
pub const HAVE: bool = false;
pub const FORCE_FLY: bool = true;
pub const SHOOT: bool = false;

pub fn install() {
    acmd::install();
    status::install();
    //agent::install();
    frame::install();

    //smashline::clone_weapon("link", "boomerang", "mario", "captoss",false);
    //smashline::clone_weapon("samus", "missile", "mario", "captoss",false);
    smashline::clone_weapon("koopajr", "cannonball", "mario", "captoss",false);
}