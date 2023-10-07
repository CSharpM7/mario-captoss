mod cappy;
mod specials;
mod capjump;
mod capdive;

pub fn install() {
    cappy::install();
    capjump::install();
    capdive::install();
    specials::install();
}