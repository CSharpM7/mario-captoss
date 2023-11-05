mod cappy;
mod capjump;
mod capcatch;
mod capdive;
mod specials;
mod appeal;
mod wait;

pub fn install() {
    cappy::install();
    capjump::install();
    capdive::install();
    capcatch::install();
    specials::install();
    appeal::install();
    wait::install();
}