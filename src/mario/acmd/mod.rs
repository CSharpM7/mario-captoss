mod cappy;
mod specials;
mod capjump;
mod capdive;
mod capcatch;

pub fn install() {
    cappy::install();
    capjump::install();
    capdive::install();
    capcatch::install();
    specials::install();
}