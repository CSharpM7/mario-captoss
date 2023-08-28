mod acmd;
mod agent;
mod status;
mod frame;
use smashline;

pub fn install() {
    acmd::install();
    status::install();
    //agent::install();
    frame::install();

    smashline::clone_weapon("link", "boomerang", "mario", "captoss",false);
    //smashline::clone_weapon("pfushigisou", "leafcutter", "mario", "captoss",false);
}